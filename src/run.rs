use anyhow::Result;
use rand::{thread_rng, Rng};
use std::io;

use crate::data::Data;
use crate::Cli;

pub fn run(cli: Cli, mut data: Data) -> Result<()> {
    if let Some(name) = cli.last_chair {
        data.change_last_chair(name)?
    }

    if let Some(name) = cli.add_member {
        data.add_member(name)
    }

    if !cli.add_members.is_empty() {
        data.add_members(cli.add_members)
    }

    if let Some(name) = cli.remove_member {
        data.remove_member(name)
    }

    if !cli.remove_members.is_empty() {
        data.remove_members(cli.remove_members)
    }

    let mut hidden_ids = Vec::new();
    if let Some(name) = cli.hide_member {
        if let Some(id) = data.get_member_id(name) {
            hidden_ids.push(id);
        }
    }

    if cli.list {
        list(&data, &hidden_ids)
    }

    if cli.run {
        execute(data, hidden_ids)
    }

    Ok(())
}

pub fn list(data: &Data, hidden_ids: &[usize]) {
    if data.members.is_empty() {
        println!("The members list is empty.")
    } else {
        println!("Members: [");
        for member in &data.members {
            println!("  {},", member)
        }
        println!("].");

        match hidden_ids.len() {
            0 => (),
            1 => println!("Will not participate: {}.", data.members[hidden_ids[0]]),
            _ => {
                println!("Will not participate: [");
                for id in hidden_ids {
                    println!("{},", data.members[*id])
                }
                println!("].")
            }
        }
    }

    if let Some(name) = &data.last_chair {
        println!("The last meeting chair was: {}", name)
    }
}

pub fn execute(mut data: Data, mut hidden_ids: Vec<usize>) {
    if data.members.is_empty() {
        println!("There is no one to be meeting chair");
    } else {
        if let Some(name) = data.last_chair.as_deref() {
            if let Some(id) = data.get_member_id(name) {
                if !hidden_ids.contains(&id) {
                    hidden_ids.push(id);
                }
            }
        }

        data.last_chair = None;
        while data.last_chair.is_none() {
            let mut rng = thread_rng();
            let len = data.members.len();

            let mut chair_index = None;
            while chair_index.is_none() {
                let random_id = rng.gen_range(0..=len);
                if !hidden_ids.contains(&random_id) {
                    chair_index = Some(random_id);
                }
            }

            let chosen_one =
                &data.members[chair_index.expect("the chosen one cannot be `None`")].clone();

            println!("The new chair is {}", chosen_one);
            println!("Continue or choose again? (y/n)");
            let mut res = String::new();

            io::stdin()
                .read_line(&mut res)
                .expect("Failed to read line");

            match res.as_str() {
                "y" | "Y" | "Yes" | "YES" | "yes" => data
                    .change_last_chair(chosen_one)
                    .expect("cannot be out of the list"),
                "n" | "N" | "No" | "NO" | "no" => data.last_chair = None,
                _ => continue,
            }
        }
    }
}