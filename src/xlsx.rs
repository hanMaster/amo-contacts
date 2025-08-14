use crate::amo::data_types::leads::DealWithContact;
use crate::error::Result;
use rust_xlsxwriter::*;
use std::fs::File;

pub struct Xlsx;

impl Xlsx {
    pub fn create(project: &str, funnel: &str, deals: Vec<DealWithContact>) -> Result<()> {
        if deals.is_empty() {
            println!("Нет данных для выгрузки");
            return Ok(());
        }

        // Create a new Excel file object.
        let mut workbook = Workbook::new();

        // Create some formats to use in the worksheet.
        let header_format = Format::new().set_bold().set_align(FormatAlign::Center);
        let align_left = Format::new().set_align(FormatAlign::Left);
        let align_center = Format::new().set_align(FormatAlign::Center);

        // Add a worksheet to the workbook.
        let worksheet = workbook.add_worksheet();

        // Set the column width for clarity.
        worksheet.set_column_width(0, 15)?;
        worksheet.set_column_width(1, 22)?;
        worksheet.set_column_width(2, 15)?;
        worksheet.set_column_width(3, 22)?;
        worksheet.set_column_width(4, 60)?;
        worksheet.set_column_width(5, 22)?;
        worksheet.set_column_width(6, 40)?;

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
            worksheet.write_with_format(row_number as RowNum, 0, project, &align_center)?;
            worksheet.write_with_format(row_number as RowNum, 1, funnel, &align_center)?;
            worksheet.write_with_format(row_number as RowNum, 2, d.deal_id, &align_center)?;
            let is_main = if d.contact.is_main { "Да" } else { "Нет" };
            worksheet.write_with_format(row_number as RowNum, 3, is_main, &align_center)?;
            worksheet.write_with_format(
                row_number as RowNum,
                4,
                format!(
                    "{} {} {}",
                    d.contact.info.first_name, d.contact.info.middle_name, d.contact.info.last_name
                ),
                &align_left,
            )?;
            worksheet.write_with_format(
                row_number as RowNum,
                5,
                &d.contact.info.phone,
                &align_left,
            )?;
            worksheet.write_with_format(
                row_number as RowNum,
                6,
                &d.contact.info.email,
                &align_left,
            )?;
            row_number += 1;
        }

        let filename = format!("{project} {funnel}.xlsx");
        // Save the file to disk.
        let file = File::create(&filename).expect("workbook file creation failed.");
        workbook.save_to_writer(&file)?;

        println!("Выгрузка в {filename} завершена успешно!");

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

        let deal1 = DealWithContact {
            deal_id: 123,
            contact: ContactInfo {
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
        };

        let deal2 = DealWithContact {
            deal_id: 345,
            contact: ContactInfo {
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
        };

        let data = vec![deal1, deal2];
        Xlsx::create(project, funnel, data).unwrap();
    }
}
