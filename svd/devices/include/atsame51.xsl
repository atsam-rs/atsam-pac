<?xml version="1.0" encoding="utf-8"?>
<xsl:stylesheet version="1.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform">
  <xsl:template match="/device/peripherals/peripheral[name='NVMCTRL']/registers/register[name='PARAM']/fields/field[name='SEE']/enumeratedValues">
  </xsl:template>

  <!-- The default SVD has an incorrect reset value - replace it. -->
  <xsl:template match="/device/peripherals/peripheral[name='SDHC0']/registers/register[name='HC1R']/resetValue">0x00</xsl:template>
  <xsl:template match="/device/peripherals/peripheral[name='SDHC0']/registers/register[name='HC1R_EMMC_MODE']/resetValue">0x00</xsl:template>
  <xsl:template match="/device/peripherals/peripheral[name='SDHC0']/registers/register[name='SISR']/resetValue">0x0000</xsl:template>
</xsl:stylesheet>
