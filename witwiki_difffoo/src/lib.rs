use std::str::FromStr;

use serde::{Deserialize, Serialize};
use slice_diff_patch::{self, Change};

type WordChanges<'a> = Vec<Change<&'a str>>;

#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub enum ChangeType {
    Remove(usize),
    Insert(usize),
    Update(usize),
}

pub enum CTParseError {
    InvalidLoc,
    InvalidType(char),
}

impl FromStr for ChangeType {
    type Err = CTParseError;
    fn from_str(v: &str) -> Result<Self, Self::Err> {
        let loc = match usize::from_str(&v[1..]) {
            Ok(x) => x,
            Err(_) => return Err(CTParseError::InvalidLoc),
        };
        match v.chars().take(1).last().expect("") {
            'i' => Ok(ChangeType::Insert(loc)),
            'u' => Ok(ChangeType::Update(loc)),
            'r' => Ok(ChangeType::Remove(loc)),
            c => Err(CTParseError::InvalidType(c)),
        }
    }
}

// Changes optimized for fitting well into the database, columns: change_type (as string), value
#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub struct WcTuple(String, Option<String>);

impl From<(ChangeType, Option<String>)> for WcTuple {
    fn from((a, b): (ChangeType, Option<String>)) -> Self {
        WcTuple(
            match a {
                ChangeType::Insert(i) => format!("i{}", i),
                ChangeType::Remove(i) => format!("r{}", i),
                ChangeType::Update(i) => format!("u{}", i),
            },
            b,
        )
    }
}

// impl Into<(String, Option<String>)> for WcTuple {
//     fn into(self) -> T {
//         // self.0.as

//         // WcTuple(ct, self.1)
//         todo!()
//     }
// }

pub struct DocDiff(String, String);

pub fn apply(base: &str, changes: &WordChanges) -> String {
    let terms = base.split(" ").collect::<Vec<&str>>();
    slice_diff_patch::patch(&terms, changes).join(" ")
}

pub fn of_diff(wc: &WordChanges) -> Vec<WcTuple> {
    wc.iter()
        .map(|x| match x {
            Change::Insert((i, s)) => (ChangeType::Insert(*i), Some((*s).to_owned())).into(),
            Change::Update((i, s)) => (ChangeType::Update(*i), Some((*s).to_owned())).into(),
            Change::Remove(i) => (ChangeType::Remove(*i), None).into(),
        })
        .collect::<Vec<WcTuple>>()
}

fn diff<'a>(a: &'a str, b: &'a str) -> WordChanges<'a> {
    let x = a.split(" ").collect::<Vec<&str>>();
    let y = b.split(" ").collect::<Vec<&str>>();
    slice_diff_patch::diff_diff(&x, &y)
}

pub fn get_changes<'a>(a: &'a str, b: &'a str) -> Vec<WcTuple> {
    of_diff(&diff(a, b))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::*;
    use slice_diff_patch::Change;
    #[test]
    fn diff_gen_and_apply() {
        let a = "i **love** dinos\n, dont you";
        let b = "he **loves** pecans\n, dont you!?";
        let delta = diff(a, b);
        assert_eq!(
            delta,
            vec![
                Change::Remove(0),
                Change::Remove(0),
                Change::Update((0, "he")),
                Change::Insert((1, "**loves**")),
                Change::Insert((2, "pecans\n,")),
                Change::Update((4, "you!?"))
            ],
            "diff is per expectation"
        );
        assert_eq!(&apply(a, &delta), &b);
    }

    #[test]
    fn serializable_diff() {
        let a = "a b c";
        let b = "a d c";
        let delta = diff(a, b);
        let out = json!(of_diff(&delta));
        assert_eq!(&out.to_string(), "[[{\"Update\":1},\"d\"]]");
    }
}
