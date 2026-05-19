#[doc = "Register `RX_CRC_EN_WR_DATA_CH%s` reader"]
pub type R = crate::R<RxCrcEnWrDataChSpec>;
#[doc = "Register `RX_CRC_EN_WR_DATA_CH%s` writer"]
pub type W = crate::W<RxCrcEnWrDataChSpec>;
#[doc = "Field `RX_CRC_EN_WR_DATA_CH` reader - This register is used to enable rx ch0 crc 32bit on/off"]
pub type RxCrcEnWrDataChR = crate::FieldReader<u32>;
#[doc = "Field `RX_CRC_EN_WR_DATA_CH` writer - This register is used to enable rx ch0 crc 32bit on/off"]
pub type RxCrcEnWrDataChW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This register is used to enable rx ch0 crc 32bit on/off"]
    #[inline(always)]
    pub fn rx_crc_en_wr_data_ch(&self) -> RxCrcEnWrDataChR {
        RxCrcEnWrDataChR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This register is used to enable rx ch0 crc 32bit on/off"]
    #[inline(always)]
    pub fn rx_crc_en_wr_data_ch(&mut self) -> RxCrcEnWrDataChW<'_, RxCrcEnWrDataChSpec> {
        RxCrcEnWrDataChW::new(self, 0)
    }
}
#[doc = "This resister is used to config ch%s crc en for every bit\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_crc_en_wr_data_ch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_crc_en_wr_data_ch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxCrcEnWrDataChSpec;
impl crate::RegisterSpec for RxCrcEnWrDataChSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_crc_en_wr_data_ch::R`](R) reader structure"]
impl crate::Readable for RxCrcEnWrDataChSpec {}
#[doc = "`write(|w| ..)` method takes [`rx_crc_en_wr_data_ch::W`](W) writer structure"]
impl crate::Writable for RxCrcEnWrDataChSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RX_CRC_EN_WR_DATA_CH%s to value 0"]
impl crate::Resettable for RxCrcEnWrDataChSpec {}
