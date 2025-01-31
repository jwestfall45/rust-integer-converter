/*
 * This file is part of Rust-Integer-Converter.
 *
 * Copyright (C) 2025 Justin Westfall
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use std::env;

// Show usage instructions
fn help() {
    print!("Error! Must pass in an unsigned integer argument.\nExample: ./rust-integer-converter 849\n");
}

/// Print hex value
/// # Arguments
/// * `num` - A 128-bit unsigned integer
fn print_hexadecimal(num: u128) {
    print!("Hex Value: 0x{:x}\n", num);
}

/// Print binary value
/// # Arguments
/// * `num` - A 128-bit unsigned integer
fn print_binary(num: u128) {
    print!("Bin Value: 0b{:b}\n", num);
}

// Main entry point
fn main() {
    // Collects the command-line arguments passed to the program into a vector of strings
    let args: Vec<String> = env::args().collect();

    match args.len() {
        // No arguments passed
        1 => {
            help();
        }
        // One argument passed
        2 => {
            // Validate string argument is an unsigned number
            if !args[1].chars().all(|c| c.is_numeric()) {
                help();
                return;
            }

            // Convert string argument to 128-bit unsigned integer type
            let val: u128 = args[1].parse::<u128>().unwrap();

            // Print values
            print!("Int Value: {}\n", val);
            print_hexadecimal(val);
            print_binary(val);
        }
        // All the other cases
        _ => {
            help();
        }
    }
}
