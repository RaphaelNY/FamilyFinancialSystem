// Prevents an additional console window on Windows in release, DO NOT REMOVE!!

// #![allow(unused)]
#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

fn main() {
    familyfinancialsystem_lib::run()
}
