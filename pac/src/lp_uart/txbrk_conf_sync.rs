#[doc = "Register `TXBRK_CONF_SYNC` reader"]
pub type R = crate::R<TxbrkConfSyncSpec>;
#[doc = "Register `TXBRK_CONF_SYNC` writer"]
pub type W = crate::W<TxbrkConfSyncSpec>;
#[doc = "Field `TX_BRK_NUM` reader - This register is used to configure the number of 0 to be sent after the process of sending data is done. It is active when txd_brk is set to 1."]
pub type TxBrkNumR = crate::FieldReader;
#[doc = "Field `TX_BRK_NUM` writer - This register is used to configure the number of 0 to be sent after the process of sending data is done. It is active when txd_brk is set to 1."]
pub type TxBrkNumW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - This register is used to configure the number of 0 to be sent after the process of sending data is done. It is active when txd_brk is set to 1."]
    #[inline(always)]
    pub fn tx_brk_num(&self) -> TxBrkNumR {
        TxBrkNumR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - This register is used to configure the number of 0 to be sent after the process of sending data is done. It is active when txd_brk is set to 1."]
    #[inline(always)]
    pub fn tx_brk_num(&mut self) -> TxBrkNumW<'_, TxbrkConfSyncSpec> {
        TxBrkNumW::new(self, 0)
    }
}
#[doc = "Tx Break character configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`txbrk_conf_sync::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbrk_conf_sync::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxbrkConfSyncSpec;
impl crate::RegisterSpec for TxbrkConfSyncSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txbrk_conf_sync::R`](R) reader structure"]
impl crate::Readable for TxbrkConfSyncSpec {}
#[doc = "`write(|w| ..)` method takes [`txbrk_conf_sync::W`](W) writer structure"]
impl crate::Writable for TxbrkConfSyncSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXBRK_CONF_SYNC to value 0x0a"]
impl crate::Resettable for TxbrkConfSyncSpec {
    const RESET_VALUE: u32 = 0x0a;
}
