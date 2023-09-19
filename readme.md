# Pico Pet

## NVM Utilization
The NVM (Non-Volatile Memory) module offers 4KiB arranged in 512 pages of 8 bytes each.

Permanent storage must be serializable to a specified number of pages, and write to an assigned page address.

Pages are addressed from hexidecimal 0x000 to 0x1ff.

| Page Start | Page End | Module |
| --- | --- | --- |
| 000 | 000 | Header |
| 001 | 001 | Settings |
<!-- proposed ranges for future NVM structs -->
<!--| 001 | 00f | Current Pet |-->
<!--| 010 | 01f | Inventory |-->

*note: If the NVM becomes corrupted, changing the const `NVM_SENTINEL` to force a header guard check failure.*
|  | **Header** |
| --- | --- |
| 0 | if set to the sentinel value, then the EEPROM contains a valid save file |
| 1 |  |
| 2 |  |
| 3 |  |
| 4 |  |
| 5 |  |
| 6 |  |
| 7 |  |

|  | **Settings** |
| --- | --- |
| 0 | User brightness | 
| 1 | User volume |
| 2 |  |
| 3 |  |
| 4 |  |
| 5 |  |
| 6 |  |
| 7 |  |


## Pin Out

```mermaid
graph LR
    subgraph RP2040
        Pin1
        Pin2
        %%Pin3[GND]
        Pin4
        Pin5
        Pin6
        Pin7
        %%Pin8[GND]
        Pin9
        Pin10
        Pin11
        Pin12
        %%Pin13[GND]
        Pin14
        Pin15
        Pin16
        Pin17
        %%Pin18[GND]
        Pin19
        Pin20
        Pin21
        Pin22
        %%Pin23[GND]
        Pin24
        Pin25
        Pin26
        Pin27
        %%Pin28[GND]
        Pin29
        Pin30
        Pin31
        Pin32
        %%Pin33[GND]
        Pin34
        Pin35
        %%Pin36[3V3_OUT]
        %%Pin37[3V3_EN]
        %%Pin38[GND]
        %%Pin39[VSYS]
        %%Pin40[VBUS]
    end

    subgraph RP2040-Tiny
        TinyPin0
        TinyPin1
        TinyPin2
        TinyPin3
        TinyPin4
        TinyPin5
        TinyPin6
        TinyPin7
        TinyPin8
        TinyPin9
        TinyPin10
        TinyPin11
        TinyPin12
        TinyPin13
        TinyPin14
        TinyPin15
        TinyPin16
        TinyPin17
        TinyPin18
        TinyPin19
        TinyPin20
        %%TinyPin21[3V3]
        %%TinyPin22[GND]
        %%TinyPin23[5V]
    end



    subgraph RTC_NVM
        RTC_SDA
        RTC_SCL
    end
    Pin1[Pin1 I2C0_SDA] --> RTC_SDA
    RTC_SDA --> TinyPin0[TinyPin0 I2C0_SDA]
    Pin2[Pin2 I2C0_SCL] --> RTC_SCL
    RTC_SCL --> TinyPin1[TinyPin1 I2C0_SCL]



    subgraph BUZZER
        SPEAKER
    end
    Pin6[Pin6 PWM_A2] --> SPEAKER
    SPEAKER --> TinyPin4[TinyPin4 PWM_A2]



    subgraph LCD
        LCD_KEY2
        LCD_KEY3
        LCD_DC
        LCD_CS
        LCD_CLK
        LCD_DIN
        LCD_RST
        LCD_BL
        LCD_KEY0
        LCD_KEY1
    end
    Pin4[Pin4 GP2] --> LCD_KEY2
    LCD_KEY2 --> TinyPin2[TinyPin2 GP2]
    Pin5[Pin5 GP3] --> LCD_KEY3
    LCD_KEY3 --> TinyPin3[TinyPin3 GP3]
    Pin11[Pin11 GP8] --> LCD_DC
    LCD_DC --> TinyPin8[TinyPin8 GP8]
    Pin12[Pin12 SPI1_CSn] --> LCD_CS
    LCD_CS --> TinyPin9[TinyPin9 SPI1_CSn]
    Pin14[Pin14 SPI1_SCK] --> LCD_CLK
    LCD_CLK --> TinyPin10[TinyPin10 SPI1_SCK]
    Pin15[Pin15 SPI1_TX] --> LCD_DIN
    LCD_DIN --> TinyPin11[TinyPin11 SPI1_TX]
    Pin16[Pin16 SPI1_RX] --> LCD_RST
    LCD_RST --> TinyPin12[TinyPin12 SPI1_RX]
    Pin17[Pin17 PWM_B6] --> LCD_BL
    LCD_BL --> TinyPin13[TinyPin13 PWM_B6]
    Pin20[Pin20 GP15] --> LCD_KEY0
    LCD_KEY0 --> TinyPin15[TinyPin15 GP15]
    Pin22[Pin22 GP17] --> LCD_KEY1
    LCD_KEY1 --> TinyPin19[TinyPin19 GP29]
```
