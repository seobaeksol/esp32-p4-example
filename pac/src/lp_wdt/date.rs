#[doc = "Register `DATE` reader"]
pub type R = crate::R<DateSpec>;
#[doc = "Register `DATE` writer"]
pub type W = crate::W<DateSpec>;
#[doc = "Field `LP_WDT_DATE` reader - need_des"]
pub type LpWdtDateR = crate::FieldReader<u32>;
#[doc = "Field `LP_WDT_DATE` writer - need_des"]
pub type LpWdtDateW<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
#[doc = "Field `CLK_EN` reader - need_des"]
pub type ClkEnR = crate::BitReader;
#[doc = "Field `CLK_EN` writer - need_des"]
pub type ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:30 - need_des"]
    #[inline(always)]
    pub fn lp_wdt_date(&self) -> LpWdtDateR {
        LpWdtDateR::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn clk_en(&self) -> ClkEnR {
        ClkEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - need_des"]
    #[inline(always)]
    pub fn lp_wdt_date(&mut self) -> LpWdtDateW<'_, DateSpec> {
        LpWdtDateW::new(self, 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn clk_en(&mut self) -> ClkEnW<'_, DateSpec> {
        ClkEnW::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DateSpec;
impl crate::RegisterSpec for DateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`date::R`](R) reader structure"]
impl crate::Readable for DateSpec {}
#[doc = "`write(|w| ..)` method takes [`date::W`](W) writer structure"]
impl crate::Writable for DateSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATE to value 0x0221_2060"]
impl crate::Resettable for DateSpec {
    const RESET_VALUE: u32 = 0x0221_2060;
}
