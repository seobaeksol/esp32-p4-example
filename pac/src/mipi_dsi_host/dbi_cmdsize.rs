#[doc = "Register `DBI_CMDSIZE` reader"]
pub type R = crate::R<DbiCmdsizeSpec>;
#[doc = "Register `DBI_CMDSIZE` writer"]
pub type W = crate::W<DbiCmdsizeSpec>;
#[doc = "Field `WR_CMD_SIZE` reader - NA"]
pub type WrCmdSizeR = crate::FieldReader<u16>;
#[doc = "Field `WR_CMD_SIZE` writer - NA"]
pub type WrCmdSizeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ALLOWED_CMD_SIZE` reader - NA"]
pub type AllowedCmdSizeR = crate::FieldReader<u16>;
#[doc = "Field `ALLOWED_CMD_SIZE` writer - NA"]
pub type AllowedCmdSizeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - NA"]
    #[inline(always)]
    pub fn wr_cmd_size(&self) -> WrCmdSizeR {
        WrCmdSizeR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - NA"]
    #[inline(always)]
    pub fn allowed_cmd_size(&self) -> AllowedCmdSizeR {
        AllowedCmdSizeR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - NA"]
    #[inline(always)]
    pub fn wr_cmd_size(&mut self) -> WrCmdSizeW<'_, DbiCmdsizeSpec> {
        WrCmdSizeW::new(self, 0)
    }
    #[doc = "Bits 16:31 - NA"]
    #[inline(always)]
    pub fn allowed_cmd_size(&mut self) -> AllowedCmdSizeW<'_, DbiCmdsizeSpec> {
        AllowedCmdSizeW::new(self, 16)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dbi_cmdsize::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbi_cmdsize::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbiCmdsizeSpec;
impl crate::RegisterSpec for DbiCmdsizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbi_cmdsize::R`](R) reader structure"]
impl crate::Readable for DbiCmdsizeSpec {}
#[doc = "`write(|w| ..)` method takes [`dbi_cmdsize::W`](W) writer structure"]
impl crate::Writable for DbiCmdsizeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DBI_CMDSIZE to value 0"]
impl crate::Resettable for DbiCmdsizeSpec {}
