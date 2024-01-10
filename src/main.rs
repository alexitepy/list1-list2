#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_mut)]

extern crate reqwest;
use chrono;

use std::f32::consts::E;
use std::fs::File;
use std::{array, fs};

use regex::Regex;
use std::env;
use std::io::{self, BufRead};
use std::path::Path;

use colored::*;
use std::{collections::BTreeSet, iter::FromIterator};
use url::Url;

fn retainings<T>(mut items: Vec<T>, to_remove: Vec<T>) -> Vec<T>
where
    T: std::cmp::Ord,
{
    let to_remove = BTreeSet::from_iter(to_remove);
    items.retain(|e| !to_remove.contains(e));

    items
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let s = format!(
        "\n\n\n{}\n{}\n{}\n{}\n{}\n\n\n",
        r#"   .-.    .-.    `-----` `-`         .-.    `----`     "#,
        r#"  / . \  / . \     | |   | |        / . \   | |  \.\`  "#,
        r#" / / \ \/ / \ \    | |   | .....`  / .-. \  | |  /./`  "#,
        r#"`-`   `--`   `-` `-----`  '-----` `-`   `-` `----`     "#,
        r#" Fastest files compare tool that you have ever sean."#
    );
    println!("{}", s.bold().green());

    let const_path = String::from("");
    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();

    let items1 =
        fs::read_to_string(format!("{}{}", const_path, "../file1.txt")).expect("Error !!!");

    let items2 =
        fs::read_to_string(format!("{}{}", const_path, "../file2.txt")).expect("Error !!!");

    let items1_ = items1.split("\n");
    let items2_ = items2.split("\n");

    println!("start :-----{}", chrono::offset::Local::now().time());
    let mut i = 0;
    for item1 in items1_ {
        vec1.push(item1);
        i = i + 1;
        println!("{}===={}===={}", "file1", i, item1);
    }
    i = 0;
    for item2 in items2_ {
        vec2.push(item2);
        i = i + 1;
        println!("{}===={}===={}", "file1", i, item2);
    }
    let mut some_x = "";
    // let counter = 0;
    i = 0;
    vec1.sort();
    vec1.dedup();

    vec2.sort();
    vec2.dedup();

    println!("{}", "---------------!!! Items sorted !!!---------------");

    println!(
        "{}",
        "---------------!!! subtraction started !!!---------------"
    );
    let vec_c = retainings(vec1, vec2);
    println!(
        "{}",
        "---------------!!! subtraction ended   !!!---------------"
    );

    fs::write(
        format!("{}{}", const_path, "../file3.txt"),
        vec_c.join("\n"),
    )
    .expect("");
    println!("end :-------{}", chrono::offset::Local::now().time());

    Ok(())
}
