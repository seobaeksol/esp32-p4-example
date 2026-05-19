#[doc = "Register `RX_CRC_DATA_EN_WR_DATA` reader"]
pub type R = crate::R<RxCrcDataEnWrDataSpec>;
#[doc = "Register `RX_CRC_DATA_EN_WR_DATA` writer"]
pub type W = crate::W<RxCrcDataEnWrDataSpec>;
#[doc = "Field `RX_CRC_DATA_EN_WR_DATA` reader - reserved"]
pub type RxCrcDataEnWrDataR = crate::FieldReader<u16>;
#[doc = "Field `RX_CRC_DATA_EN_WR_DATA` writer - reserved"]
pub type RxCrcDataEnWrDataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - reserved"]
    #[inline(always)]
    pub fn rx_crc_data_en_wr_data(&self) -> RxCrcDataEnWrDataR {
        RxCrcDataEnWrDataR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - reserved"]
    #[inline(always)]
    pub fn rx_crc_data_en_wr_data(&mut self) -> RxCrcDataEnWrDataW<'_, RxCrcDataEnWrDataSpec> {
        RxCrcDataEnWrDataW::new(self, 0)
    }
}
#[doc = "This register is used to config crc data_8bit en\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_crc_data_en_wr_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_crc_data_en_wr_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxCrcDataEnWrDataSpec;
impl crate::RegisterSpec for RxCrcDataEnWrDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_crc_data_en_wr_data::R`](R) reader structure"]
impl crate::Readable for RxCrcDataEnWrDataSpec {}
#[doc = "`write(|w| ..)` method takes [`rx_crc_data_en_wr_data::W`](W) writer structure"]
impl crate::Writable for RxCrcDataEnWrDataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RX_CRC_DATA_EN_WR_DATA to value 0"]
impl crate::Resettable for RxCrcDataEnWrDataSpec {}
