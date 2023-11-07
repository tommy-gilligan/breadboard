use heapless::sorted_linked_list::{SortedLinkedList, Max, LinkedIndexU8};
use heapless::Vec;
use embedded_graphics_core::pixelcolor::Rgb888;

const COLOR_PAIRS: [[Rgb888; 2]; 3] = [
    [Rgb888::new(255, 255, 0), Rgb888::new(255, 165, 0)],
    [Rgb888::new(255, 0, 0), Rgb888::new(0, 255, 0)],
    [Rgb888::new(0, 0, 255), Rgb888::new(255, 0, 255)],
];

type Connection = (usize, usize, Rgb888);

pub struct ViewModel {
    top: Vec<SortedLinkedList<Connection, LinkedIndexU8, Max, 20>, 5>,
    current_row: Option<SortedLinkedList<Connection, LinkedIndexU8, Max, 20>>,
    row_index: usize,
}
use crate::model::Connections;

impl From<&Connections> for ViewModel {
    fn from(model: &Connections) -> ViewModel {
        let mut model = model.clone();
        let mut packed: Vec<SortedLinkedList<Connection, LinkedIndexU8, Max, 20>, 5> = heapless::Vec::new();

        while model.iter().count() > 0 {
            let popped = *model.iter().next().unwrap();
            model.remove(popped);

            let packing: SortedLinkedList<Connection, LinkedIndexU8, Max, 20> = SortedLinkedList::new_u8();
            packed.push(packing).unwrap();
        }

        ViewModel { top: packed, current_row: None, row_index: 0 }
    }
}

impl Iterator for ViewModel {
    type Item = (usize, Connection);

    fn next(&mut self) -> Option<Self::Item> {
        if !self.top.is_empty() && (self.current_row.is_none() || (self.current_row.is_some() && self.current_row.as_mut().unwrap().is_empty()))  {
            self.current_row = self.top.pop();
        }

        None
    }
}


