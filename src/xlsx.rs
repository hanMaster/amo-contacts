use crate::amo::data_types::leads::{Ids, LeadInfo, ProfitWithContact};

use crate::error::Result;
use crate::profit::get_ru_object_type;
use rust_xlsxwriter::*;
use std::fs::File;

pub struct Xlsx;

impl Xlsx {
    pub fn create(deals: Vec<LeadInfo>) -> Result<()> {
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
        worksheet.set_column_width(3, 15)?;
        worksheet.set_column_width(4, 22)?;
        worksheet.set_column_width(5, 22)?;
        worksheet.set_column_width(6, 22)?;

        // Write a string without formatting.
        worksheet.write_with_format(0, 0, "Проект", &header_format)?;
        worksheet.write_with_format(0, 1, "Воронка", &header_format)?;
        worksheet.write_with_format(0, 2, "№ сделки", &header_format)?;
        worksheet.write_with_format(0, 3, "Тип договора", &header_format)?;
        worksheet.write_with_format(0, 4, "№ дома", &header_format)?;
        worksheet.write_with_format(0, 5, "Тип недвижимости", &header_format)?;
        worksheet.write_with_format(0, 6, "№ объекта", &header_format)?;

        let mut row_number = 1;

        let mut project = "".to_string();

        for d in deals {
            worksheet.write_with_format(
                row_number as RowNum,
                0,
                d.project,
                &align_center,
            )?;

            worksheet.write_with_format(row_number as RowNum, 1, "Передача ЖК", &align_center)?;

            // № сделки
            worksheet.write_with_format(
                row_number as RowNum,
                2,
                d.lead_id,
                &align_center,
            )?;

            worksheet.write_with_format(row_number as RowNum, 3, "", &align_center)?;

            worksheet.write_with_format(
                row_number as RowNum,
                4,
                d.house,
                &align_center,
            )?;

            worksheet.write_with_format(
                row_number as RowNum,
                5,
                get_ru_object_type(&d.property_type),
                &align_center,
            )?;

            worksheet.write_with_format(
                row_number as RowNum,
                6,
                d.property_num,
                &align_center,
            )?;

            row_number += 1;
        }

        let filename = "peredacha.xlsx".to_string();
        // Save the file to disk.
        let file = File::create(&filename).expect("workbook file creation failed.");
        workbook.save_to_writer(&file)?;

        println!("Выгрузка в {filename} завершена успешно!");

        Ok(())
    }

    pub fn create_ids(pairs: Vec<Ids>) -> Result<()> {
        if pairs.is_empty() {
            println!("Нет данных для выгрузки");
            return Ok(());
        }

        // Create a new Excel file object.
        let mut workbook = Workbook::new();

        // Create some formats to use in the worksheet.
        let header_format = Format::new().set_bold().set_align(FormatAlign::Center);
        let align_left = Format::new().set_align(FormatAlign::Left);

        // Add a worksheet to the workbook.
        let worksheet = workbook.add_worksheet();

        // Set the column width for clarity.
        worksheet.set_column_width(0, 22)?;
        worksheet.set_column_width(1, 22)?;

        // Write a string without formatting.
        worksheet.write_with_format(0, 0, "№ сделки", &header_format)?;
        worksheet.write_with_format(0, 1, "№ объекта", &header_format)?;

        let mut row_number = 1;

        for d in pairs {
            worksheet.write_with_format(row_number as RowNum, 0, d.lead_id, &align_left)?;
            worksheet.write_with_format(row_number as RowNum, 1, d.profit_id, &align_left)?;
            row_number += 1;
        }

        // Save the file to disk.
        let file = File::create("pairs.xlsx").expect("workbook file creation failed.");
        workbook.save_to_writer(&file)?;

        println!("Выгрузка завершена успешно!");

        Ok(())
    }
}

