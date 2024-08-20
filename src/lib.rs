include!("ljx8_if_async.rs");
#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn get_serial_number() -> Result<(), Box<dyn std::error::Error>> {
        let mut ljx8_if_async = LJX8IFAsync::ljx8_if_ethernet_open("192.168.0.2", 24691).await?;
        let (controller_serial, head_serial) = ljx8_if_async.ljx8_if_get_serial_number().await?;
        println!("controller_serial: {:?}", controller_serial);
        println!("head_serial: {:?}", head_serial);
        let (pn_sensor_temperature, pn_processor_temperature, pn_case_temperature) =
            ljx8_if_async.ljx8_if_get_head_temperature().await?;
        println!("pnSensorTemperature: {:?}", pn_sensor_temperature);
        println!("pnProcessorTemperature: {:?}", pn_processor_temperature);
        println!("pnCaseTemperature: {:?}", pn_case_temperature);
        let result = ljx8_if_async.ljx8_if_communication_close().await?;
        assert_eq!(result, 0);
        Ok(())
    }
    #[tokio::test]
    async fn get_head_temperature() -> Result<(), Box<dyn std::error::Error>> {
        let mut ljx8_if_async = LJX8IFAsync::ljx8_if_ethernet_open("192.168.0.2", 24691).await?;
        let (pn_sensor_temperature, pn_processor_temperature, pn_case_temperature) =
            ljx8_if_async.ljx8_if_get_head_temperature().await?;
        println!("pnSensorTemperature: {:?}", pn_sensor_temperature);
        println!("pnProcessorTemperature: {:?}", pn_processor_temperature);
        println!("pnCaseTemperature: {:?}", pn_case_temperature);
        let result = ljx8_if_async.ljx8_if_communication_close().await?;
        assert_eq!(result, 0);
        Ok(())
    }
}
