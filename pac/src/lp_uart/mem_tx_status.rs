#[doc = "Register `MEM_TX_STATUS` reader"]
pub type R = crate::R<MemTxStatusSpec>;
#[doc = "Field `TX_SRAM_WADDR` reader - This register stores the offset write address in Tx-SRAM."]
pub type TxSramWaddrR = crate::FieldReader;
#[doc = "Field `TX_SRAM_RADDR` reader - This register stores the offset read address in Tx-SRAM."]
pub type TxSramRaddrR = crate::FieldReader;
impl R {
    #[doc = "Bits 3:7 - This register stores the offset write address in Tx-SRAM."]
    #[inline(always)]
    pub fn tx_sram_waddr(&self) -> TxSramWaddrR {
        TxSramWaddrR::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 12:16 - This register stores the offset read address in Tx-SRAM."]
    #[inline(always)]
    pub fn tx_sram_raddr(&self) -> TxSramRaddrR {
        TxSramRaddrR::new(((self.bits >> 12) & 0x1f) as u8)
    }
}
#[doc = "Tx-SRAM write and read offset address.\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_tx_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemTxStatusSpec;
impl crate::RegisterSpec for MemTxStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_tx_status::R`](R) reader structure"]
impl crate::Readable for MemTxStatusSpec {}
#[doc = "`reset()` method sets MEM_TX_STATUS to value 0"]
impl crate::Resettable for MemTxStatusSpec {}
