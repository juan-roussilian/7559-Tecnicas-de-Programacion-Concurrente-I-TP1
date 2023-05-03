mod constants;
mod contenedor;
mod contenedor_recarga;
mod pedido;
mod pedidos_parser;
mod trait_contenedor_cafetera;
mod dispensador;
mod console_logger;

use crate::constants::*;
use crate::contenedor::Contenedor;
use crate::contenedor_recarga::ContenedorRecarga;
use crate::pedidos_parser::PedidosParser;
use std::sync::{Arc, RwLock};
use std::{fs, thread};
use std::collections::HashMap;
use std::thread::JoinHandle;
use crate::console_logger::ConsoleLogger;
use crate::dispensador::Dispensador;

fn main() {
    let mut pedidos_string = "".to_string();
    if let Ok(string) = fs::read_to_string(PATH_ARCHIVO_JSON_PEDIDOS){
        pedidos_string = string;
    }else{
        println!("Error al intentar leer el archivo de pedidos. Revisar path en constants.rs");
    }

    let mut pedidos = Arc::new(RwLock::new(vec![]));
    if let Ok(lista_pedidos) = PedidosParser::new(&pedidos_string).obtener_pedidos() {
        pedidos = Arc::new(RwLock::new(lista_pedidos));
    };
    let mut consumos:HashMap<String,u32> = HashMap::new();
    consumos.insert("granos".to_string(), 0);
    consumos.insert("cafe".to_string(),0);
    consumos.insert("leche".to_string(),0);
    consumos.insert("espuma".to_string(),0);
    consumos.insert("agua".to_string(),0);
    consumos.insert("red".to_string(),0);
    consumos.insert("cacao".to_string(),0);
    let consumos_arc = Arc::new(RwLock::new(consumos));
    let contador_pedidos_preparados = Arc::new(RwLock::new(0));



    let contenedor_cafe =
        match crear_arc_lock_contenedor(CAPACIDAD_CAFE_MOLIDO, CAPACIDAD_CAFE_GRANO) {
            Ok(contenedor_arc) => contenedor_arc,
            Err(e) => {
                println!("{}",e);
                return;
            }
        };
    let contenedor_espuma =
        match crear_arc_lock_contenedor(CAPACIDAD_LECHE_ESPUMA, CAPACIDAD_LECHE_FRIA) {
            Ok(contenedor_arc) => contenedor_arc,
            Err(e) => {
                println!("{}",e);
                return;
            }
        };
    let contenedor_agua =
        match crear_arc_lock_contenedor(CAPACIDAD_AGUA_CALIENTE, f64::INFINITY as u32) {
            Ok(contenedor_arc) => contenedor_arc,
            Err(e) => {
                println!("{}",e);
                return;
            }
        };
    let contenedor_cacao = match crear_arc_lock_contenedor(CAPACIDAD_CACAO, 0) {
        Ok(contenedor_arc) => contenedor_arc,
        Err(e) => {
            println!("{}",e);
            return;
        }
    };

    let mut hilos: Vec<JoinHandle<()>> = vec![];

    let logger = ConsoleLogger::new(
        contenedor_cafe.clone(),
        contenedor_agua.clone(),
        contenedor_espuma.clone(),
        contenedor_cacao.clone(),
        consumos_arc.clone(),
        contador_pedidos_preparados.clone()
    );

    hilos.push(thread::spawn(move || logger.loggear_estadisticas()));

    let mut dispensadores: Vec<JoinHandle<()>> = (0..CANTIDAD_DISPENSADORES)
        .map(|id| {
            let mut dispensador = Dispensador::new(id as u32,contenedor_cafe.clone(),contenedor_agua.clone(),contenedor_espuma.clone(),contenedor_cacao.clone());
            let lista_pedidos = pedidos.clone();
            let consumos = consumos_arc.clone();
            let contador_pedidos = contador_pedidos_preparados.clone();

            thread::spawn(move || dispensador.producir_bebidas(lista_pedidos, consumos, contador_pedidos))
        })
        .collect();
    hilos.append(&mut dispensadores);

    hilos.into_iter()
        .flat_map(|x| x.join())
        .for_each(drop);
}

pub fn crear_arc_lock_contenedor(
    capacidad_contenedor: u32,
    capacidad_contenedor_rec: u32,
) -> Result<Arc<RwLock<Contenedor>>, String> {
    let contenedor_final;
    if capacidad_contenedor_rec > 0 {
        match ContenedorRecarga::new(capacidad_contenedor_rec, capacidad_contenedor_rec) {
            Ok(contenedor_recarga) => {
                match Contenedor::new(
                    capacidad_contenedor,
                    capacidad_contenedor,
                    Some(contenedor_recarga),
                ) {
                    Ok(contenedor) => {
                        contenedor_final = Arc::new(RwLock::new(contenedor));
                        Ok(contenedor_final)
                    }
                    Err(e) => Err(format!(
                        "[Error]{}: Revisar valores de constantes en constants.rs",
                        e
                    )),
                }
            }
            Err(e) => Err(format!(
                "[Error]{}: Revisar valores de constantes en constants.rs",
                e
            )),
        }
    } else {
        match Contenedor::new(capacidad_contenedor, capacidad_contenedor, None) {
            Ok(contenedor) => {
                contenedor_final = Arc::new(RwLock::new(contenedor));
                Ok(contenedor_final)
            }
            Err(e) => Err(format!(
                "[Error]{}: Revisar valores de constantes en constants.rs",
                e
            )),
        }
    }
}
