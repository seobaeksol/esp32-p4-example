#[doc = "Register `CLKDIV` reader"]
pub type R = crate::R<ClkdivSpec>;
#[doc = "Register `CLKDIV` writer"]
pub type W = crate::W<ClkdivSpec>;
#[doc = "Field `CLK_DIVIDER0` reader - Clock divider0 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on."]
pub type ClkDivider0R = crate::FieldReader;
#[doc = "Field `CLK_DIVIDER0` writer - Clock divider0 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on."]
pub type ClkDivider0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CLK_DIVIDER1` reader - Clock divider1 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on."]
pub type ClkDivider1R = crate::FieldReader;
#[doc = "Field `CLK_DIVIDER1` writer - Clock divider1 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on."]
pub type ClkDivider1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CLK_DIVIDER2` reader - Clock divider2 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on."]
pub type ClkDivider2R = crate::FieldReader;
#[doc = "Field `CLK_DIVIDER2` writer - Clock divider2 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on."]
pub type ClkDivider2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CLK_DIVIDER3` reader - Clock divider3 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on."]
pub type ClkDivider3R = crate::FieldReader;
#[doc = "Field `CLK_DIVIDER3` writer - Clock divider3 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on."]
pub type ClkDivider3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Clock divider0 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on."]
    #[inline(always)]
    pub fn clk_divider0(&self) -> ClkDivider0R {
        ClkDivider0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Clock divider1 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on."]
    #[inline(always)]
    pub fn clk_divider1(&self) -> ClkDivider1R {
        ClkDivider1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Clock divider2 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on."]
    #[inline(always)]
    pub fn clk_divider2(&self) -> ClkDivider2R {
        ClkDivider2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Clock divider3 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on."]
    #[inline(always)]
    pub fn clk_divider3(&self) -> ClkDivider3R {
        ClkDivider3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock divider0 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on."]
    #[inline(always)]
    pub fn clk_divider0(&mut self) -> ClkDivider0W<'_, ClkdivSpec> {
        ClkDivider0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Clock divider1 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on."]
    #[inline(always)]
    pub fn clk_divider1(&mut self) -> ClkDivider1W<'_, ClkdivSpec> {
        ClkDivider1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Clock divider2 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on."]
    #[inline(always)]
    pub fn clk_divider2(&mut self) -> ClkDivider2W<'_, ClkdivSpec> {
        ClkDivider2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Clock divider3 value. Clock divisor is 2*n, where n = 0 bypasses the divider (divisor of 1). For example, a value of 1 means divided by 2*1 = 2, a value of 0xFF means divided by 2*255 = 510, and so on."]
    #[inline(always)]
    pub fn clk_divider3(&mut self) -> ClkDivider3W<'_, ClkdivSpec> {
        ClkDivider3W::new(self, 24)
    }
}
#[doc = "Clock divider configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`clkdiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkdiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkdivSpec;
impl crate::RegisterSpec for ClkdivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkdiv::R`](R) reader structure"]
impl crate::Readable for ClkdivSpec {}
#[doc = "`write(|w| ..)` method takes [`clkdiv::W`](W) writer structure"]
impl crate::Writable for ClkdivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLKDIV to value 0"]
impl crate::Resettable for ClkdivSpec {}
