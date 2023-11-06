// Problem
// https://leetcode.com/problems/seat-reservation-manager

use std::collections::BinaryHeap;

struct SeatManager {
    available_seats: BinaryHeap<i32>,
}

impl SeatManager {
    fn new(n: i32) -> Self {
        let mut available_seats = BinaryHeap::new();
        for seat_number in 1..=n {
            available_seats.push(seat_number);
        }
        SeatManager { available_seats }
    }

    fn reserve(&mut self) -> i32 {
        if let Some(seat_number) = self.available_seats.pop() {
            seat_number
        } else {
            0
        }
    }

    fn unreserve(&mut self, seat_number: i32) {
        self.available_seats.push(seat_number);
    }
    fn get_list(&self) -> BinaryHeap<i32> {
        self.available_seats.clone()
    }
}

fn main() {
    let mut seat_manager = SeatManager::new(5);
    seat_manager.reserve();
    println!("{:?}", seat_manager.get_list());
    seat_manager.reserve();
    println!("{:?}", seat_manager.get_list());
    seat_manager.unreserve(2);
    println!("{:?}", seat_manager.get_list());
    seat_manager.reserve();
    println!("{:?}", seat_manager.get_list());
    seat_manager.reserve();
    println!("{:?}", seat_manager.get_list());
    seat_manager.reserve();
    println!("{:?}", seat_manager.get_list());
    seat_manager.reserve();
    println!("{:?}", seat_manager.get_list());
    seat_manager.unreserve(5);
    println!("{:?}", seat_manager.get_list());
}

/*
* Optimized implementation

use std::collections::BinaryHeap;
use std::iter::FromIterator;

struct SeatManager {
    available_seats: BinaryHeap<i32>,
}

impl SeatManager {
    fn new(n: i32) -> Self {
        SeatManager { available_seats: BinaryHeap::<i32>::from_iter(-n..0) }
    }

    fn reserve(&mut self) -> i32 {
        -self.available_seats.pop().unwrap()
    }

    fn unreserve(&mut self, seat_number: i32) {
        self.available_seats.push(-seat_number);
    }
}

*/
