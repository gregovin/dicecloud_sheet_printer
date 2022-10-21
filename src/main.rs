use std::env;
use genpdf::{Element, Alignment};
use genpdf::{elements,fonts, style};
use dicecloud_sheet_printer::{generate_pdf,holding_structs::*};
use serde_json::Value;
fn main() {
    let mut doc = generate_pdf();
    let username = "gregovin".to_string();
    let psw = "stuff".to_string();
    
    doc.push(
        elements::Paragraph::new("Dungeons and Dragons")
            .aligned(Alignment::Left)
            .styled(style::Style::new().bold().with_font_size(20)),
    );
    let mut table = elements::TableLayout::new(vec![1,2]);
    table.set_cell_decorator(elements::FrameCellDecorator::new(true, true, false));
    table
        .row()
        .element(
            elements::LinearLayout::vertical()
                .element(elements::Break::new(0.5))
                .element(elements::Paragraph::new("Charname").aligned(Alignment::Center))
                .padded(1)
        )
        .element(
            elements::LinearLayout::vertical()
                .element(elements::Paragraph::new("stuff"))
                .element(elements::Break::new(0.25))
                .element(elements::Paragraph::new("Things"))
                .padded(1)
        )
        .push()
        .expect("Invalid Table Row");
    doc.push(table);
    doc.render_to_file("./character_sheet.pdf").expect("Failed to write output file");
}
