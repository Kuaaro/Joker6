use std::sync::mpsc::{Receiver, Sender};
use crate::joker_execution_unit::{JokerExecutionUnitForward, StandardEncryptionJokerExecutionUnit};

pub fn single_jue_per_thread(from: Receiver<u8>, key: &[u8; 256], to: Sender<u8>) {
    let mut jeu = StandardEncryptionJokerExecutionUnit::new(key);
    let mut index: u8 = 255;

    while let Ok(value) = from.recv() {
        index += 1;
        to.send(jeu.put(value, index));
    }

    let mut out: [u8; 18] = [0u8; 18];
    jeu.close(&mut out, index);

    for i in out {
        to.send(i);
    }
}

pub fn multiple_jue_per_thread(from: Receiver<u8>, key: &[[u8; 256]], to: Sender<u8>) {
    let mut jeus: Vec<StandardEncryptionJokerExecutionUnit> = Vec::with_capacity(key.len());
    let mut index: u8 = 255;
    let mut input: u8;
    let mut closing_values = [0u8; 18];

    for i in key {
        jeus.push(StandardEncryptionJokerExecutionUnit::new(i))
    }

    while let Ok(recv) = from.recv() {
        index += 1;
        input = recv;

        for jeu in jeus.iter_mut() {
            input = jeu.put(input, index);
        }

        to.send(input);
    }

    for closing_jeu in 0..jeus.len() {
        jeus[closing_jeu].close(&mut closing_values, index);

        for closed_value in closing_values {
            index += 1;
            input = closed_value;

            for jeu in closing_jeu+1..jeus.len() {
                input = jeus[jeu].put(input, index);
            }

            to.send(input);
        }
    }
}