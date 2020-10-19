use std::time::Duration;
use std::process::Command;
use WatchEvent::{HasChange, NoChange};

fn split_md(md_list: Vec<u8>) -> Vec<String> {
    let mut heads = vec![0_usize];
    let mut tails = vec![];
    let len = md_list.len();
    md_list.iter().zip(0..len).for_each(|(&md, index)| {
        if md == 10u8 {
            heads.push(index + 1);
        } else if md == 46u8 {
            tails.push(index);
        }
    });
    heads.pop();

    let mut result = vec![];
    for (head, tail) in heads.into_iter().zip(tails) {
        let md = String::from_utf8_lossy(&md_list[head..tail]).to_string();
        result.push(md);
    }

    result
}

#[derive(PartialEq, Debug)]
pub enum WatchEvent {
    HasChange(Vec<String>),
    NoChange,
}

async fn is_change(old_list: &[String]) -> WatchEvent {
    let output = Command::new("ls")
        .arg("static/markdown")
        .output()
        .expect("no such file or director");

    if output.status.success() {
        let markdowns = split_md(output.stdout);
        if markdowns == old_list {
            NoChange
        } else {
            HasChange(markdowns)
        }
    } else {
        old_list;
        NoChange
    }
}

#[cfg(test)]
mod test {
    use crate::watch::{split_md, is_change};
    use std::process::Command;
    use futures::executor::block_on;
    use crate::watch::WatchEvent::{HasChange, NoChange};

    #[test]
    fn ls_command() {
        let output = Command::new("ls")
            .arg("static/markdown")
            .output()
            .expect("no such file or director");

        println!("status: {}", output.status);
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        println!("markdowns: {:?}", split_md(output.stdout));

        assert!(output.status.success());
    }

    #[test]
    fn is_change_ts() {
        let output = Command::new("ls")
            .arg("static/markdown")
            .output()
            .expect("no such file or director");
        let mut old_markdowns = split_md(output.stdout);

        let event = block_on(is_change(&old_markdowns));

        assert_eq!(NoChange, event);

        old_markdowns.push("phantom".to_string());

        let event = block_on(is_change(&old_markdowns));

        assert_ne!(event, NoChange, "old list = {:?}\nnew list={:?}", old_markdowns, event);
    }
}
