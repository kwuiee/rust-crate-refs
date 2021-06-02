use std::error::Error;

use xlsxwriter::*;

fn main() -> Result<(), Box<dyn Error>> {
    let workbook = Workbook::new("simple1.xlsx");
    let mut format1 = workbook.add_format().set_font_color(FormatColor::Red);

    let mut format2 = workbook
        .add_format()
        .set_font_color(FormatColor::Blue)
        .set_underline(FormatUnderline::Single);

    let mut format3 = workbook
        .add_format()
        .set_font_color(FormatColor::Green)
        .set_align(FormatAlignment::CenterAcross)
        .set_align(FormatAlignment::VerticalCenter);

    let mut sheet1 = workbook.add_worksheet(None)?;
    sheet1.write_string(0, 0, "Red text", Some(&format1))?;
    sheet1.write_number(0, 1, 20., None)?;
    sheet1.write_formula_num(1, 0, "=10+B1", None, 30.)?;
    sheet1.write_url(
        1,
        1,
        "https://github.com/informationsea/xlsxwriter-rs",
        Some(&format2),
    )?;
    sheet1.merge_range(2, 0, 3, 2, "Hello, world", Some(&format3))?;

    sheet1.set_selection(1, 0, 1, 2);
    sheet1.set_tab_color(FormatColor::Cyan);
    workbook.close()?;
    Ok(())
}
