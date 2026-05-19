#[doc = "Register `OUT_PRI` reader"]
pub type R = crate::R<OutPriSpec>;
#[doc = "Register `OUT_PRI` writer"]
pub type W = crate::W<OutPriSpec>;
#[doc = "Field `TX_PRI` reader - The priority of Tx channel0. The larger of the value the higher of the priority."]
pub type TxPriR = crate::FieldReader;
#[doc = "Field `TX_PRI` writer - The priority of Tx channel0. The larger of the value the higher of the priority."]
pub type TxPriW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TX_CH_ARB_WEIGH` reader - The weight of Tx channel0"]
pub type TxChArbWeighR = crate::FieldReader;
#[doc = "Field `TX_CH_ARB_WEIGH` writer - The weight of Tx channel0"]
pub type TxChArbWeighW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TX_ARB_WEIGH_OPT_DIR` reader - 0: mean not optimazation weight function ,1: mean optimazation"]
pub type TxArbWeighOptDirR = crate::BitReader;
#[doc = "Field `TX_ARB_WEIGH_OPT_DIR` writer - 0: mean not optimazation weight function ,1: mean optimazation"]
pub type TxArbWeighOptDirW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - The priority of Tx channel0. The larger of the value the higher of the priority."]
    #[inline(always)]
    pub fn tx_pri(&self) -> TxPriR {
        TxPriR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - The weight of Tx channel0"]
    #[inline(always)]
    pub fn tx_ch_arb_weigh(&self) -> TxChArbWeighR {
        TxChArbWeighR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - 0: mean not optimazation weight function ,1: mean optimazation"]
    #[inline(always)]
    pub fn tx_arb_weigh_opt_dir(&self) -> TxArbWeighOptDirR {
        TxArbWeighOptDirR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - The priority of Tx channel0. The larger of the value the higher of the priority."]
    #[inline(always)]
    pub fn tx_pri(&mut self) -> TxPriW<'_, OutPriSpec> {
        TxPriW::new(self, 0)
    }
    #[doc = "Bits 4:7 - The weight of Tx channel0"]
    #[inline(always)]
    pub fn tx_ch_arb_weigh(&mut self) -> TxChArbWeighW<'_, OutPriSpec> {
        TxChArbWeighW::new(self, 4)
    }
    #[doc = "Bit 8 - 0: mean not optimazation weight function ,1: mean optimazation"]
    #[inline(always)]
    pub fn tx_arb_weigh_opt_dir(&mut self) -> TxArbWeighOptDirW<'_, OutPriSpec> {
        TxArbWeighOptDirW::new(self, 8)
    }
}
#[doc = "Priority register of Tx channel0.\n\nYou can [`read`](crate::Reg::read) this register and get [`out_pri::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_pri::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutPriSpec;
impl crate::RegisterSpec for OutPriSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_pri::R`](R) reader structure"]
impl crate::Readable for OutPriSpec {}
#[doc = "`write(|w| ..)` method takes [`out_pri::W`](W) writer structure"]
impl crate::Writable for OutPriSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OUT_PRI to value 0"]
impl crate::Resettable for OutPriSpec {}
