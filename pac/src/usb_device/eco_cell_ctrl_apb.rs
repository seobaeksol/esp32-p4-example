#[doc = "Register `ECO_CELL_CTRL_APB` reader"]
pub type R = crate::R<EcoCellCtrlApbSpec>;
#[doc = "Register `ECO_CELL_CTRL_APB` writer"]
pub type W = crate::W<EcoCellCtrlApbSpec>;
#[doc = "Field `RDN_RESULT_APB` reader - Reserved."]
pub type RdnResultApbR = crate::BitReader;
#[doc = "Field `RDN_ENA_APB` reader - Reserved."]
pub type RdnEnaApbR = crate::BitReader;
#[doc = "Field `RDN_ENA_APB` writer - Reserved."]
pub type RdnEnaApbW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reserved."]
    #[inline(always)]
    pub fn rdn_result_apb(&self) -> RdnResultApbR {
        RdnResultApbR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved."]
    #[inline(always)]
    pub fn rdn_ena_apb(&self) -> RdnEnaApbR {
        RdnEnaApbR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Reserved."]
    #[inline(always)]
    pub fn rdn_ena_apb(&mut self) -> RdnEnaApbW<'_, EcoCellCtrlApbSpec> {
        RdnEnaApbW::new(self, 1)
    }
}
#[doc = "Reserved.\n\nYou can [`read`](crate::Reg::read) this register and get [`eco_cell_ctrl_apb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eco_cell_ctrl_apb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EcoCellCtrlApbSpec;
impl crate::RegisterSpec for EcoCellCtrlApbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eco_cell_ctrl_apb::R`](R) reader structure"]
impl crate::Readable for EcoCellCtrlApbSpec {}
#[doc = "`write(|w| ..)` method takes [`eco_cell_ctrl_apb::W`](W) writer structure"]
impl crate::Writable for EcoCellCtrlApbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ECO_CELL_CTRL_APB to value 0"]
impl crate::Resettable for EcoCellCtrlApbSpec {}
