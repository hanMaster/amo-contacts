use crate::amo::data_types::leads::DealWithContacts;
use crate::error::Result;
use rust_xlsxwriter::*;
use std::fs::File;

pub struct Xlsx;

impl Xlsx {
    pub fn create(project: &str, funnel: &str, deals: Vec<DealWithContacts>) -> Result<()> {
        // Create a new Excel file object.
        let mut workbook = Workbook::new();

        // Create some formats to use in the worksheet.
        let header_format = Format::new().set_bold().set_align(FormatAlign::Center);
        let row_format = Format::new().set_align(FormatAlign::Center);
        // let date_format = Format::new().set_num_format("dd.mm.yyyy");

        // Add a worksheet to the workbook.
        let worksheet = workbook.add_worksheet();

        // Set the column width for clarity.
        worksheet.set_column_width(0, 15)?;
        worksheet.set_column_width(1, 22)?;
        worksheet.set_column_width(2, 15)?;
        worksheet.set_column_width(3, 22)?;
        worksheet.set_column_width(4, 32)?;
        worksheet.set_column_width(5, 22)?;
        worksheet.set_column_width(6, 22)?;

        // Write a string without formatting.
        worksheet.write_with_format(0, 0, "Проект", &header_format)?;
        worksheet.write_with_format(0, 1, "Воронка", &header_format)?;
        worksheet.write_with_format(0, 2, "№ сделки", &header_format)?;
        worksheet.write_with_format(0, 3, "Основной контакт", &header_format)?;
        worksheet.write_with_format(0, 4, "ФИО", &header_format)?;
        worksheet.write_with_format(0, 5, "Телефон", &header_format)?;
        worksheet.write_with_format(0, 6, "Email", &header_format)?;

        let mut row_number = 1;

        for d in deals {
            for c in d.contacts {
                worksheet.write_with_format((row_number) as RowNum, 0, project, &row_format)?;
                worksheet.write_with_format((row_number) as RowNum, 1, funnel, &row_format)?;
                worksheet.write_with_format((row_number) as RowNum, 2, d.deal_id, &row_format)?;
                let is_main = if c.is_main { "Да" } else { "Нет" };
                worksheet.write_with_format((row_number) as RowNum, 3, is_main, &row_format)?;
                worksheet.write_with_format(
                    (row_number) as RowNum,
                    4,
                    &format!(
                        "{} {} {}",
                        c.info.first_name, c.info.middle_name, c.info.last_name
                    ),
                    &row_format,
                )?;
                worksheet.write_with_format(
                    (row_number) as RowNum,
                    5,
                    &c.info.phone,
                    &row_format,
                )?;
                worksheet.write_with_format(
                    (row_number) as RowNum,
                    6,
                    &c.info.email,
                    &row_format,
                )?;
                row_number += 1;
            }
        }

        // Save the file to disk.
        let file = File::create(format!("{project}_{funnel}.xlsx"))
            .expect("workbook file creation failed.");
        workbook.save_to_writer(&file)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::amo::data_types::leads::{Contact, ContactInfo};
    #[test]
    fn test_create_worksheet() {
        let project = "Формат";
        let funnel = "Передача ЖК14";

        let deal = DealWithContacts {
            deal_id: 123,
            contacts: vec![
                ContactInfo {
                    is_main: true,
                    info: Contact {
                        id: 123,
                        owner: false,
                        first_name: "Василий".to_string(),
                        middle_name: "Иванович".to_string(),
                        last_name: "Пупкин".to_string(),
                        phone: "+79244567895".to_string(),
                        email: "abc@mail.ru".to_string(),
                    },
                },
                ContactInfo {
                    is_main: false,
                    info: Contact {
                        id: 456,
                        owner: false,
                        first_name: "Мария".to_string(),
                        middle_name: "Ивановна".to_string(),
                        last_name: "Пупкина".to_string(),
                        phone: "+79244567896".to_string(),
                        email: "def@mail.ru".to_string(),
                    },
                },
            ],
        };

        let data = vec![deal];
        Xlsx::create(project, funnel, data).unwrap();
    }
}
