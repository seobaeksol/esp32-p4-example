#[doc = "Register `RX_CRC_DATA_EN_ADDR` reader"]
pub type R = crate::R<RxCrcDataEnAddrSpec>;
#[doc = "Register `RX_CRC_DATA_EN_ADDR` writer"]
pub type W = crate::W<RxCrcDataEnAddrSpec>;
#[doc = "Field `RX_CRC_DATA_EN_ADDR` reader - reserved"]
pub type RxCrcDataEnAddrR = crate::FieldReader<u32>;
#[doc = "Field `RX_CRC_DATA_EN_ADDR` writer - reserved"]
pub type RxCrcDataEnAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - reserved"]
    #[inline(always)]
    pub fn rx_crc_data_en_addr(&self) -> RxCrcDataEnAddrR {
        RxCrcDataEnAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - reserved"]
    #[inline(always)]
    pub fn rx_crc_data_en_addr(&mut self) -> RxCrcDataEnAddrW<'_, RxCrcDataEnAddrSpec> {
        RxCrcDataEnAddrW::new(self, 0)
    }
}
#[doc = "This register is used to config addr of crc data_8bit en\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_crc_data_en_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_crc_data_en_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxCrcDataEnAddrSpec;
impl crate::RegisterSpec for RxCrcDataEnAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_crc_data_en_addr::R`](R) reader structure"]
impl crate::Readable for RxCrcDataEnAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`rx_crc_data_en_addr::W`](W) writer structure"]
impl crate::Writable for RxCrcDataEnAddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RX_CRC_DATA_EN_ADDR to value 0"]
impl crate::Resettable for RxCrcDataEnAddrSpec {}
