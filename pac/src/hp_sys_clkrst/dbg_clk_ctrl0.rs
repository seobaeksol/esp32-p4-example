#[doc = "Register `DBG_CLK_CTRL0` reader"]
pub type R = crate::R<DbgClkCtrl0Spec>;
#[doc = "Register `DBG_CLK_CTRL0` writer"]
pub type W = crate::W<DbgClkCtrl0Spec>;
#[doc = "Field `DBG_CH0_SEL` reader - Reserved"]
pub type DbgCh0SelR = crate::FieldReader;
#[doc = "Field `DBG_CH0_SEL` writer - Reserved"]
pub type DbgCh0SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DBG_CH1_SEL` reader - Reserved"]
pub type DbgCh1SelR = crate::FieldReader;
#[doc = "Field `DBG_CH1_SEL` writer - Reserved"]
pub type DbgCh1SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DBG_CH2_SEL` reader - Reserved"]
pub type DbgCh2SelR = crate::FieldReader;
#[doc = "Field `DBG_CH2_SEL` writer - Reserved"]
pub type DbgCh2SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DBG_CH0_DIV_NUM` reader - Reserved"]
pub type DbgCh0DivNumR = crate::FieldReader;
#[doc = "Field `DBG_CH0_DIV_NUM` writer - Reserved"]
pub type DbgCh0DivNumW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    pub fn dbg_ch0_sel(&self) -> DbgCh0SelR {
        DbgCh0SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Reserved"]
    #[inline(always)]
    pub fn dbg_ch1_sel(&self) -> DbgCh1SelR {
        DbgCh1SelR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Reserved"]
    #[inline(always)]
    pub fn dbg_ch2_sel(&self) -> DbgCh2SelR {
        DbgCh2SelR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Reserved"]
    #[inline(always)]
    pub fn dbg_ch0_div_num(&self) -> DbgCh0DivNumR {
        DbgCh0DivNumR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    pub fn dbg_ch0_sel(&mut self) -> DbgCh0SelW<'_, DbgClkCtrl0Spec> {
        DbgCh0SelW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Reserved"]
    #[inline(always)]
    pub fn dbg_ch1_sel(&mut self) -> DbgCh1SelW<'_, DbgClkCtrl0Spec> {
        DbgCh1SelW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Reserved"]
    #[inline(always)]
    pub fn dbg_ch2_sel(&mut self) -> DbgCh2SelW<'_, DbgClkCtrl0Spec> {
        DbgCh2SelW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Reserved"]
    #[inline(always)]
    pub fn dbg_ch0_div_num(&mut self) -> DbgCh0DivNumW<'_, DbgClkCtrl0Spec> {
        DbgCh0DivNumW::new(self, 24)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_clk_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_clk_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgClkCtrl0Spec;
impl crate::RegisterSpec for DbgClkCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg_clk_ctrl0::R`](R) reader structure"]
impl crate::Readable for DbgClkCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`dbg_clk_ctrl0::W`](W) writer structure"]
impl crate::Writable for DbgClkCtrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DBG_CLK_CTRL0 to value 0x03ff_ffff"]
impl crate::Resettable for DbgClkCtrl0Spec {
    const RESET_VALUE: u32 = 0x03ff_ffff;
}
