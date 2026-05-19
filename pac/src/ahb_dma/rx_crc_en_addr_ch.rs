#[doc = "Register `RX_CRC_EN_ADDR_CH%s` reader"]
pub type R = crate::R<RxCrcEnAddrChSpec>;
#[doc = "Register `RX_CRC_EN_ADDR_CH%s` writer"]
pub type W = crate::W<RxCrcEnAddrChSpec>;
#[doc = "Field `RX_CRC_EN_ADDR_CH` reader - reserved"]
pub type RxCrcEnAddrChR = crate::FieldReader<u32>;
#[doc = "Field `RX_CRC_EN_ADDR_CH` writer - reserved"]
pub type RxCrcEnAddrChW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - reserved"]
    #[inline(always)]
    pub fn rx_crc_en_addr_ch(&self) -> RxCrcEnAddrChR {
        RxCrcEnAddrChR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - reserved"]
    #[inline(always)]
    pub fn rx_crc_en_addr_ch(&mut self) -> RxCrcEnAddrChW<'_, RxCrcEnAddrChSpec> {
        RxCrcEnAddrChW::new(self, 0)
    }
}
#[doc = "This register is used to config ch%s crc en addr\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_crc_en_addr_ch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_crc_en_addr_ch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxCrcEnAddrChSpec;
impl crate::RegisterSpec for RxCrcEnAddrChSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_crc_en_addr_ch::R`](R) reader structure"]
impl crate::Readable for RxCrcEnAddrChSpec {}
#[doc = "`write(|w| ..)` method takes [`rx_crc_en_addr_ch::W`](W) writer structure"]
impl crate::Writable for RxCrcEnAddrChSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RX_CRC_EN_ADDR_CH%s to value 0"]
impl crate::Resettable for RxCrcEnAddrChSpec {}
