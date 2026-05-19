#[doc = "Register `DBG_CLK_CTRL1` reader"]
pub type R = crate::R<DbgClkCtrl1Spec>;
#[doc = "Register `DBG_CLK_CTRL1` writer"]
pub type W = crate::W<DbgClkCtrl1Spec>;
#[doc = "Field `DBG_CH1_DIV_NUM` reader - Reserved"]
pub type DbgCh1DivNumR = crate::FieldReader;
#[doc = "Field `DBG_CH1_DIV_NUM` writer - Reserved"]
pub type DbgCh1DivNumW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DBG_CH2_DIV_NUM` reader - Reserved"]
pub type DbgCh2DivNumR = crate::FieldReader;
#[doc = "Field `DBG_CH2_DIV_NUM` writer - Reserved"]
pub type DbgCh2DivNumW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DBG_CH0_EN` reader - Reserved"]
pub type DbgCh0EnR = crate::BitReader;
#[doc = "Field `DBG_CH0_EN` writer - Reserved"]
pub type DbgCh0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_CH1_EN` reader - Reserved"]
pub type DbgCh1EnR = crate::BitReader;
#[doc = "Field `DBG_CH1_EN` writer - Reserved"]
pub type DbgCh1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_CH2_EN` reader - Reserved"]
pub type DbgCh2EnR = crate::BitReader;
#[doc = "Field `DBG_CH2_EN` writer - Reserved"]
pub type DbgCh2EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    pub fn dbg_ch1_div_num(&self) -> DbgCh1DivNumR {
        DbgCh1DivNumR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Reserved"]
    #[inline(always)]
    pub fn dbg_ch2_div_num(&self) -> DbgCh2DivNumR {
        DbgCh2DivNumR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Reserved"]
    #[inline(always)]
    pub fn dbg_ch0_en(&self) -> DbgCh0EnR {
        DbgCh0EnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Reserved"]
    #[inline(always)]
    pub fn dbg_ch1_en(&self) -> DbgCh1EnR {
        DbgCh1EnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Reserved"]
    #[inline(always)]
    pub fn dbg_ch2_en(&self) -> DbgCh2EnR {
        DbgCh2EnR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    pub fn dbg_ch1_div_num(&mut self) -> DbgCh1DivNumW<'_, DbgClkCtrl1Spec> {
        DbgCh1DivNumW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Reserved"]
    #[inline(always)]
    pub fn dbg_ch2_div_num(&mut self) -> DbgCh2DivNumW<'_, DbgClkCtrl1Spec> {
        DbgCh2DivNumW::new(self, 8)
    }
    #[doc = "Bit 16 - Reserved"]
    #[inline(always)]
    pub fn dbg_ch0_en(&mut self) -> DbgCh0EnW<'_, DbgClkCtrl1Spec> {
        DbgCh0EnW::new(self, 16)
    }
    #[doc = "Bit 17 - Reserved"]
    #[inline(always)]
    pub fn dbg_ch1_en(&mut self) -> DbgCh1EnW<'_, DbgClkCtrl1Spec> {
        DbgCh1EnW::new(self, 17)
    }
    #[doc = "Bit 18 - Reserved"]
    #[inline(always)]
    pub fn dbg_ch2_en(&mut self) -> DbgCh2EnW<'_, DbgClkCtrl1Spec> {
        DbgCh2EnW::new(self, 18)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg_clk_ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg_clk_ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgClkCtrl1Spec;
impl crate::RegisterSpec for DbgClkCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg_clk_ctrl1::R`](R) reader structure"]
impl crate::Readable for DbgClkCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`dbg_clk_ctrl1::W`](W) writer structure"]
impl crate::Writable for DbgClkCtrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DBG_CLK_CTRL1 to value 0x0303"]
impl crate::Resettable for DbgClkCtrl1Spec {
    const RESET_VALUE: u32 = 0x0303;
}
