#[doc = "Register `TX_CRC_DATA_EN_WR_DATA` reader"]
pub type R = crate::R<TxCrcDataEnWrDataSpec>;
#[doc = "Register `TX_CRC_DATA_EN_WR_DATA` writer"]
pub type W = crate::W<TxCrcDataEnWrDataSpec>;
#[doc = "Field `TX_CRC_DATA_EN_WR_DATA` reader - reserved"]
pub type TxCrcDataEnWrDataR = crate::FieldReader<u16>;
#[doc = "Field `TX_CRC_DATA_EN_WR_DATA` writer - reserved"]
pub type TxCrcDataEnWrDataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - reserved"]
    #[inline(always)]
    pub fn tx_crc_data_en_wr_data(&self) -> TxCrcDataEnWrDataR {
        TxCrcDataEnWrDataR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - reserved"]
    #[inline(always)]
    pub fn tx_crc_data_en_wr_data(&mut self) -> TxCrcDataEnWrDataW<'_, TxCrcDataEnWrDataSpec> {
        TxCrcDataEnWrDataW::new(self, 0)
    }
}
#[doc = "This register is used to config crc data_8bit en\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_crc_data_en_wr_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_crc_data_en_wr_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxCrcDataEnWrDataSpec;
impl crate::RegisterSpec for TxCrcDataEnWrDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_crc_data_en_wr_data::R`](R) reader structure"]
impl crate::Readable for TxCrcDataEnWrDataSpec {}
#[doc = "`write(|w| ..)` method takes [`tx_crc_data_en_wr_data::W`](W) writer structure"]
impl crate::Writable for TxCrcDataEnWrDataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TX_CRC_DATA_EN_WR_DATA to value 0"]
impl crate::Resettable for TxCrcDataEnWrDataSpec {}
