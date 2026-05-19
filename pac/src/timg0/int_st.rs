#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<IntStSpec>;
#[doc = "Field `T(0-1)` reader - The masked interrupt status bit for the TIMG_T%s_INT interrupt."]
pub type TR = crate::BitReader;
#[doc = "Field `WDT` reader - The masked interrupt status bit for the TIMG_WDT_INT interrupt."]
pub type WdtR = crate::BitReader;
impl R {
    #[doc = "The masked interrupt status bit for the TIMG_T(0-1)_INT interrupt."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `T0` field.</div>"]
    #[inline(always)]
    pub fn t(&self, n: u8) -> TR {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        TR::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "The masked interrupt status bit for the TIMG_T(0-1)_INT interrupt."]
    #[inline(always)]
    pub fn t_iter(&self) -> impl Iterator<Item = TR> + '_ {
        (0..2).map(move |n| TR::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - The masked interrupt status bit for the TIMG_T0_INT interrupt."]
    #[inline(always)]
    pub fn t0(&self) -> TR {
        TR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The masked interrupt status bit for the TIMG_T1_INT interrupt."]
    #[inline(always)]
    pub fn t1(&self) -> TR {
        TR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The masked interrupt status bit for the TIMG_WDT_INT interrupt."]
    #[inline(always)]
    pub fn wdt(&self) -> WdtR {
        WdtR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntStSpec;
impl crate::RegisterSpec for IntStSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st::R`](R) reader structure"]
impl crate::Readable for IntStSpec {}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for IntStSpec {}
