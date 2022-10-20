use std::env;
use genpdf::{Element, Alignment};
use genpdf::{elements,fonts, style};
fn main() {
    let font = fonts::from_files("./Roboto","Roboto",None).expect("Failed to load font");
    let mut doc = genpdf::Document::new(font);
    doc.set_title("Character Sheet");
    doc.set_minimal_conformance();
    doc.set_line_spacing(1.25);

    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(10);
    decorator.set_header(|page| {
        let mut layout = elements::LinearLayout::vertical();
        if page>1 {
            layout.push(
                elements::Paragraph::new(format!("Page {}", page)).aligned(Alignment::Right),
            );
            layout.push(elements::Break::new(1));
        }
        layout.styled(style::Style::new().with_font_size(10))
    });
    doc.set_page_decorator(decorator);

    #[cfg(feature = "hyphenation")]
    {
        use hyphenation::Load;

        doc.set_hyphenator(
            hyphenation::Standard::from_embedded(hyphenation::Language::EnglishUS)
                .expect("Failed to load hyphenation data"),
        );
    }
    doc.push(
        elements::Paragraph::new("Dungeons and Dragons")
            .aligned(Alignment::Left)
            .styled(style::Style::new().bold().with_font_size(20)),
    );
    doc.render_to_file("./character_sheet.pdf").expect("Failed to write output file");
}
