<?xml version="1.0" encoding="utf-8"?>
<xsl:stylesheet version="1.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform">
  <!-- Some of the lesser used interrupt vectors are missing -->
  <xsl:template match="/device/peripherals/peripheral[name='SUPC']/addressBlock">
    <xsl:copy-of select="."/>
    <interrupt>
      <name>SUPC</name>
      <value>0</value>
    </interrupt>
  </xsl:template>
  <xsl:template match="/device/peripherals/peripheral[name='RSTC']/addressBlock">
    <xsl:copy-of select="."/>
    <interrupt>
      <name>RSTC</name>
      <value>1</value>
    </interrupt>
  </xsl:template>
  <xsl:template match="/device/peripherals/peripheral[name='RTC']/addressBlock">
    <xsl:copy-of select="."/>
    <interrupt>
      <name>RTC</name>
      <value>2</value>
    </interrupt>
  </xsl:template>
  <xsl:template match="/device/peripherals/peripheral[name='RTT']/addressBlock">
    <xsl:copy-of select="."/>
    <interrupt>
      <name>RTT</name>
      <value>3</value>
    </interrupt>
  </xsl:template>
  <xsl:template match="/device/peripherals/peripheral[name='WDT']/addressBlock">
    <xsl:copy-of select="."/>
    <interrupt>
      <name>WDT</name>
      <value>4</value>
    </interrupt>
  </xsl:template>
  <!-- The CHIPID enumerations for the SAM4SxA and SAM3SxA (and SAM4SxB/SAM3SxB and SAM4SxC/SAM3SxC) are the same.  We remove the
  second enumerated values here since the two chips share the same IDs and created duplicates in the created enumeration.
  -->
  <xsl:template match="/device/peripherals/peripheral[name='CHIPID']/registers/register[name='CIDR']/fields/field[name='ARCH']/enumeratedValues/enumeratedValue[name='SAM4SxA']">
  </xsl:template>
  <xsl:template match="/device/peripherals/peripheral[name='CHIPID']/registers/register[name='CIDR']/fields/field[name='ARCH']/enumeratedValues/enumeratedValue[name='SAM4SxB']">
  </xsl:template>
  <xsl:template match="/device/peripherals/peripheral[name='CHIPID']/registers/register[name='CIDR']/fields/field[name='ARCH']/enumeratedValues/enumeratedValue[name='SAM4SxC']">
  </xsl:template>
</xsl:stylesheet>
