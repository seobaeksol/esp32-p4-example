#[doc = "Register `RX_CRC_DATA_EN_ADDR_CH%s` reader"]
pub type R = crate::R<RxCrcDataEnAddrChSpec>;
#[doc = "Register `RX_CRC_DATA_EN_ADDR_CH%s` writer"]
pub type W = crate::W<RxCrcDataEnAddrChSpec>;
#[doc = "Field `RX_CRC_DATA_EN_ADDR_CH` reader - reserved"]
pub type RxCrcDataEnAddrChR = crate::FieldReader<u32>;
#[doc = "Field `RX_CRC_DATA_EN_ADDR_CH` writer - reserved"]
pub type RxCrcDataEnAddrChW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - reserved"]
    #[inline(always)]
    pub fn rx_crc_data_en_addr_ch(&self) -> RxCrcDataEnAddrChR {
        RxCrcDataEnAddrChR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - reserved"]
    #[inline(always)]
    pub fn rx_crc_data_en_addr_ch(&mut self) -> RxCrcDataEnAddrChW<'_, RxCrcDataEnAddrChSpec> {
        RxCrcDataEnAddrChW::new(self, 0)
    }
}
#[doc = "This register is used to config addr of crc data_8bit en\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_crc_data_en_addr_ch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_crc_data_en_addr_ch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxCrcDataEnAddrChSpec;
impl crate::RegisterSpec for RxCrcDataEnAddrChSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_crc_data_en_addr_ch::R`](R) reader structure"]
impl crate::Readable for RxCrcDataEnAddrChSpec {}
#[doc = "`write(|w| ..)` method takes [`rx_crc_data_en_addr_ch::W`](W) writer structure"]
impl crate::Writable for RxCrcDataEnAddrChSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RX_CRC_DATA_EN_ADDR_CH%s to value 0"]
impl crate::Resettable for RxCrcDataEnAddrChSpec {}
