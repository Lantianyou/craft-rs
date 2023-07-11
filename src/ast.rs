use crate::token::Token;
use std::fs::File;
use std::io::{BufWriter, Write};

fn define_ast(output_dir: String, base_name: &str, types: Vec<&str>) {
    let path = format!("{}/{}.rs", output_dir, base_name.to_lowercase());
    let file = File::create(path).expect("Unable to create file");
    let mut writer = BufWriter::new(file);
    writer.write_all(b"use crate::token::Token;\n\n").unwrap();
    writer
        .write_all(format!("trait {} {{\n", base_name).as_bytes())
        .unwrap();
    writer.write_all(b"}\n\n").unwrap();
    define_visitor(&mut writer, base_name, &types);
    types.iter().for_each(|t| {
        let first_colon = t.find(":").unwrap();
        let name = t[0..first_colon].trim();
        let fields = t[first_colon + 1..].trim();
        define_type(&mut writer, base_name, name, fields);
    });
    writer.write_all(b"\n").unwrap();
    writer.write_all(b"impl Pastry for Beignet {\n").unwrap();
    writer.write_all(b"}\n\n").unwrap();
}

fn define_type(writer: &mut BufWriter<File>, base_name: &str, name: &str, field_list: &str) {
    writer
        .write_all(format!("struct {} {{\n", name).as_bytes())
        .unwrap();
    let fields = field_list.split(",").collect::<Vec<&str>>();
    println!("{:?}", fields);
    fields.iter().for_each(|f| {
        println!("{:?}", f.split(":").collect::<Vec<&str>>());
        let value = f.split(":").collect::<Vec<&str>>()[0].trim();
        let type_name = f.split(":").collect::<Vec<&str>>()[1].trim();
        writer
            .write_all(format!("{}: {},\n", value, type_name).as_bytes())
            .unwrap();
    });
    writer.write_all(b"}\n\n").unwrap();
    writer
        .write_all(format!("impl {} for {} {{\n", base_name, name).as_bytes())
        .unwrap();
    writer.write_all(b"}\n\n").unwrap();
}

fn define_visitor(writer: &mut BufWriter<File>, base_name: &str, types: &Vec<&str>) {
    writer.write_all(b"trait PastryVisitor {\n").unwrap();
    types.iter().for_each(|t| {
        let type_name = t.split(":").collect::<Vec<&str>>()[0].trim();
        writer
            .write_all(
                format!(
                    "    fn visit_{}_{}({}: {});\n",
                    type_name.to_lowercase(),
                    base_name.to_lowercase(),
                    type_name.to_lowercase(),
                    base_name
                )
                .as_bytes(),
            )
            .unwrap();
    });
    writer.write_all(b"}\n\n").unwrap();
}

trait Expr {}

struct Unary<'a, T: Expr> {
    operator: Token<'a>,
    right: T,
}

struct Binary<'a, T: Expr> {
    left: T,
    operator: Token<'a>,
    right: T,
}

// struct Grouping<'a, T: Expr> {}

trait PastryVisitor {
    fn visit_binary_expr(&mut self, expr: &Beignet);
    fn visit_unary_expr(&mut self, expr: &Cruller);
}

trait Pastry {
    fn accept(&self, visitor: &mut dyn PastryVisitor);
}

struct Beignet {}

impl Pastry for Beignet {
    fn accept(&self, visitor: &mut dyn PastryVisitor) {
        visitor.visit_binary_expr(self)
    }
}

struct Cruller {}

impl Pastry for Cruller {
    fn accept(&self, visitor: &mut dyn PastryVisitor) {
        visitor.visit_unary_expr(self)
    }
}

pub fn main() {
    define_ast(
        "src".to_string(),
        "Expr",
        vec![
            "Binary   : left: Expr, operator: Token, right: Expr",
            "Grouping : expression: Expr",
            "Literal  : value: Literal",
            "Unary    : operator: Token, right: Expr",
        ],
    );
}
