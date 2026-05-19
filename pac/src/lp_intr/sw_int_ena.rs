#[doc = "Register `SW_INT_ENA` reader"]
pub type R = crate::R<SwIntEnaSpec>;
#[doc = "Register `SW_INT_ENA` writer"]
pub type W = crate::W<SwIntEnaSpec>;
#[doc = "Field `LP_SW_INT_ENA` reader - need_des"]
pub type LpSwIntEnaR = crate::BitReader;
#[doc = "Field `LP_SW_INT_ENA` writer - need_des"]
pub type LpSwIntEnaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lp_sw_int_ena(&self) -> LpSwIntEnaR {
        LpSwIntEnaR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lp_sw_int_ena(&mut self) -> LpSwIntEnaW<'_, SwIntEnaSpec> {
        LpSwIntEnaW::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`sw_int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwIntEnaSpec;
impl crate::RegisterSpec for SwIntEnaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sw_int_ena::R`](R) reader structure"]
impl crate::Readable for SwIntEnaSpec {}
#[doc = "`write(|w| ..)` method takes [`sw_int_ena::W`](W) writer structure"]
impl crate::Writable for SwIntEnaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SW_INT_ENA to value 0"]
impl crate::Resettable for SwIntEnaSpec {}
