#[doc = "Register `LCD_LATTER_CMD_VAL` reader"]
pub type R = crate::R<LcdLatterCmdValSpec>;
#[doc = "Register `LCD_LATTER_CMD_VAL` writer"]
pub type W = crate::W<LcdLatterCmdValSpec>;
#[doc = "Field `LCD_LATTER_CMD_VALUE` reader - The LCD write command value of latter cmd cycle."]
pub type LcdLatterCmdValueR = crate::FieldReader<u32>;
#[doc = "Field `LCD_LATTER_CMD_VALUE` writer - The LCD write command value of latter cmd cycle."]
pub type LcdLatterCmdValueW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The LCD write command value of latter cmd cycle."]
    #[inline(always)]
    pub fn lcd_latter_cmd_value(&self) -> LcdLatterCmdValueR {
        LcdLatterCmdValueR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The LCD write command value of latter cmd cycle."]
    #[inline(always)]
    pub fn lcd_latter_cmd_value(&mut self) -> LcdLatterCmdValueW<'_, LcdLatterCmdValSpec> {
        LcdLatterCmdValueW::new(self, 0)
    }
}
#[doc = "LCD config register.\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_latter_cmd_val::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_latter_cmd_val::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdLatterCmdValSpec;
impl crate::RegisterSpec for LcdLatterCmdValSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_latter_cmd_val::R`](R) reader structure"]
impl crate::Readable for LcdLatterCmdValSpec {}
#[doc = "`write(|w| ..)` method takes [`lcd_latter_cmd_val::W`](W) writer structure"]
impl crate::Writable for LcdLatterCmdValSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCD_LATTER_CMD_VAL to value 0"]
impl crate::Resettable for LcdLatterCmdValSpec {}
