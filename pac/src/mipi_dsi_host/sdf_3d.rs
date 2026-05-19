#[doc = "Register `SDF_3D` reader"]
pub type R = crate::R<Sdf3dSpec>;
#[doc = "Register `SDF_3D` writer"]
pub type W = crate::W<Sdf3dSpec>;
#[doc = "Field `MODE_3D` reader - NA"]
pub type Mode3dR = crate::FieldReader;
#[doc = "Field `MODE_3D` writer - NA"]
pub type Mode3dW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FORMAT_3D` reader - NA"]
pub type Format3dR = crate::FieldReader;
#[doc = "Field `FORMAT_3D` writer - NA"]
pub type Format3dW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SECOND_VSYNC` reader - NA"]
pub type SecondVsyncR = crate::BitReader;
#[doc = "Field `SECOND_VSYNC` writer - NA"]
pub type SecondVsyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RIGHT_FIRST` reader - NA"]
pub type RightFirstR = crate::BitReader;
#[doc = "Field `RIGHT_FIRST` writer - NA"]
pub type RightFirstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEND_3D_CFG` reader - NA"]
pub type Send3dCfgR = crate::BitReader;
#[doc = "Field `SEND_3D_CFG` writer - NA"]
pub type Send3dCfgW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - NA"]
    #[inline(always)]
    pub fn mode_3d(&self) -> Mode3dR {
        Mode3dR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - NA"]
    #[inline(always)]
    pub fn format_3d(&self) -> Format3dR {
        Format3dR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    pub fn second_vsync(&self) -> SecondVsyncR {
        SecondVsyncR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - NA"]
    #[inline(always)]
    pub fn right_first(&self) -> RightFirstR {
        RightFirstR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16 - NA"]
    #[inline(always)]
    pub fn send_3d_cfg(&self) -> Send3dCfgR {
        Send3dCfgR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - NA"]
    #[inline(always)]
    pub fn mode_3d(&mut self) -> Mode3dW<'_, Sdf3dSpec> {
        Mode3dW::new(self, 0)
    }
    #[doc = "Bits 2:3 - NA"]
    #[inline(always)]
    pub fn format_3d(&mut self) -> Format3dW<'_, Sdf3dSpec> {
        Format3dW::new(self, 2)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    pub fn second_vsync(&mut self) -> SecondVsyncW<'_, Sdf3dSpec> {
        SecondVsyncW::new(self, 4)
    }
    #[doc = "Bit 5 - NA"]
    #[inline(always)]
    pub fn right_first(&mut self) -> RightFirstW<'_, Sdf3dSpec> {
        RightFirstW::new(self, 5)
    }
    #[doc = "Bit 16 - NA"]
    #[inline(always)]
    pub fn send_3d_cfg(&mut self) -> Send3dCfgW<'_, Sdf3dSpec> {
        Send3dCfgW::new(self, 16)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`sdf_3d::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdf_3d::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sdf3dSpec;
impl crate::RegisterSpec for Sdf3dSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdf_3d::R`](R) reader structure"]
impl crate::Readable for Sdf3dSpec {}
#[doc = "`write(|w| ..)` method takes [`sdf_3d::W`](W) writer structure"]
impl crate::Writable for Sdf3dSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SDF_3D to value 0"]
impl crate::Resettable for Sdf3dSpec {}
