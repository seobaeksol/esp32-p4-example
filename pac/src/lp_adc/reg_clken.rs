#[doc = "Register `REG_CLKEN` reader"]
pub type R = crate::R<RegClkenSpec>;
#[doc = "Register `REG_CLKEN` writer"]
pub type W = crate::W<RegClkenSpec>;
#[doc = "Field `CLK_EN` reader - N/A"]
pub type ClkEnR = crate::BitReader;
#[doc = "Field `CLK_EN` writer - N/A"]
pub type ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - N/A"]
    #[inline(always)]
    pub fn clk_en(&self) -> ClkEnR {
        ClkEnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - N/A"]
    #[inline(always)]
    pub fn clk_en(&mut self) -> ClkEnW<'_, RegClkenSpec> {
        ClkEnW::new(self, 0)
    }
}
#[doc = "N/A\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_clken::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_clken::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegClkenSpec;
impl crate::RegisterSpec for RegClkenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_clken::R`](R) reader structure"]
impl crate::Readable for RegClkenSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_clken::W`](W) writer structure"]
impl crate::Writable for RegClkenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CLKEN to value 0"]
impl crate::Resettable for RegClkenSpec {}
