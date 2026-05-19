#[doc = "Register `RX_CRC_EN_WR_DATA` reader"]
pub type R = crate::R<RxCrcEnWrDataSpec>;
#[doc = "Register `RX_CRC_EN_WR_DATA` writer"]
pub type W = crate::W<RxCrcEnWrDataSpec>;
#[doc = "Field `RX_CRC_EN_WR_DATA` reader - This register is used to enable rx ch0 crc 32bit on/off"]
pub type RxCrcEnWrDataR = crate::FieldReader<u32>;
#[doc = "Field `RX_CRC_EN_WR_DATA` writer - This register is used to enable rx ch0 crc 32bit on/off"]
pub type RxCrcEnWrDataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This register is used to enable rx ch0 crc 32bit on/off"]
    #[inline(always)]
    pub fn rx_crc_en_wr_data(&self) -> RxCrcEnWrDataR {
        RxCrcEnWrDataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This register is used to enable rx ch0 crc 32bit on/off"]
    #[inline(always)]
    pub fn rx_crc_en_wr_data(&mut self) -> RxCrcEnWrDataW<'_, RxCrcEnWrDataSpec> {
        RxCrcEnWrDataW::new(self, 0)
    }
}
#[doc = "This resister is used to config ch0 crc en for every bit\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_crc_en_wr_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_crc_en_wr_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxCrcEnWrDataSpec;
impl crate::RegisterSpec for RxCrcEnWrDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_crc_en_wr_data::R`](R) reader structure"]
impl crate::Readable for RxCrcEnWrDataSpec {}
#[doc = "`write(|w| ..)` method takes [`rx_crc_en_wr_data::W`](W) writer structure"]
impl crate::Writable for RxCrcEnWrDataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RX_CRC_EN_WR_DATA to value 0"]
impl crate::Resettable for RxCrcEnWrDataSpec {}
