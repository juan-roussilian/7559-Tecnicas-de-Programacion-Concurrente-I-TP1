use crate::trait_contenedor_cafetera::ContenedorCafetera;
use std::thread;
use std::time::Duration;

pub struct ContenedorRecarga {
    cantidad: u32,
    capacidad: u32,
}

impl ContenedorRecarga {
    pub fn new(cantidad: u32, capacidad: u32) -> Result<Self, String> {
        if cantidad <= capacidad {
            Ok(ContenedorRecarga {
                cantidad,
                capacidad,
            })
        } else {
            Err("La cantidad con la que se inicializa el contenedor de recarga no puede ser mayor a la capacidad".into())
        }
    }
    pub fn obtener_max_contenido(&mut self) -> u32 {
        // TODO: Ademas debe ser llamado por el hilo "recargador" que estara verificando una condvar
        // de la cantidad que hay en el contenedor.
        let max_cantidad = self.cantidad;
        thread::sleep(Duration::from_millis(max_cantidad as u64));
        self.cantidad = 0;
        max_cantidad
    }
}
impl ContenedorCafetera for ContenedorRecarga {
    fn obtener_contenido(&mut self, cantidad_obtener: u32) -> Option<u32> {
        //un milisegundo equivale a 1gr o 1ml
        if cantidad_obtener <= self.cantidad {
            thread::sleep(Duration::from_millis(cantidad_obtener as u64));
            self.cantidad -= cantidad_obtener;
            Some(cantidad_obtener)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contenedor_recarga_debe_tener_cantidad_menor_o_igual_a_capacidad() {
        assert!(ContenedorRecarga::new(1, 0).is_err());
    }

    #[test]
    fn contenedor_recarga_con_cantidad_500_obtener_contenido_10_devuelve_10() {
        let mut contenedor =
            ContenedorRecarga::new(500, 500).expect("Fallo la creacion del contenedor recarga");
        assert_eq!(contenedor.obtener_contenido(10), Some(10))
    }

    #[test]
    fn contenedor_recarga_con_cantidad_2_obtener_contenido_10_devuelve_none() {
        let mut contenedor =
            ContenedorRecarga::new(2, 500).expect("Fallo la creacion del contenedor recarga");
        assert_eq!(contenedor.obtener_contenido(10), None)
    }

    #[test]
    fn contenedor_recarga_lleno_con_capacidad_300_obtener_max_contenido_devuelve_300() {
        let mut contenedor =
            ContenedorRecarga::new(300, 300).expect("Fallo la creacion del contenedor recarga");
        assert_eq!(contenedor.obtener_max_contenido(), 300)
    }

    #[test]
    fn contenedor_recarga_con_cantidad_50_y_capacidad_300_obtener_max_contenido_devuelve_50() {
        let mut contenedor =
            ContenedorRecarga::new(50, 300).expect("Fallo la creacion del contenedor recarga");
        assert_eq!(contenedor.obtener_max_contenido(), 50)
    }

    #[test]
    fn contenedor_recarga_vacio_obtener_max_contenido_devuelve_0() {
        let mut contenedor =
            ContenedorRecarga::new(0, 300).expect("Fallo la creacion del contenedor recarga");
        assert_eq!(contenedor.obtener_max_contenido(), 0)
    }
}
