<?xml version="1.0" encoding="utf-8"?>
<xsl:stylesheet version="1.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform">
  <!-- The CHIPID enumerations for the SAM4SxA and SAM3SxA (and SAM4SxB/SAM3SxB and SAM4SxC/SAM3SxC) are the same.  We remove the
  second enumerated values here since the two chips share the same IDs and created duplicates in the created enumeration.
  -->
  <xsl:template match="/device/peripherals/peripheral[name='CHIPID']/registers/register[name='CIDR']/fields/field[name='ARCH']/enumeratedValues/enumeratedValue[name='SAM4SxA']">
  </xsl:template>
  <xsl:template match="/device/peripherals/peripheral[name='CHIPID']/registers/register[name='CIDR']/fields/field[name='ARCH']/enumeratedValues/enumeratedValue[name='SAM4SxB']">
  </xsl:template>
  <xsl:template match="/device/peripherals/peripheral[name='CHIPID']/registers/register[name='CIDR']/fields/field[name='ARCH']/enumeratedValues/enumeratedValue[name='SAM4SxC']">
  </xsl:template>

  <!-- Add the PASSWD enumeration to SUPC[CR] and SUPC[MR] registers' KEY field -->
  <xsl:template match="/device/peripherals/peripheral[name='SUPC']/registers/register[name='CR']/fields/field[name='KEY']|/device/peripherals/peripheral[name='SUPC']/registers/register[name='MR']/fields/field[name='KEY']">
    <!-- Copy the element -->
    <xsl:copy>
      <!-- And everything inside it -->
      <xsl:apply-templates select="@* | *"/> 
      <!-- Add new node for the enumeratedValues -->
      <xsl:element name="enumeratedValues">
        <enumeratedValue>
          <name>PASSWD</name>
          <description>Writing any other value in this field aborts the write operation.</description>
          <value>0xA5</value>
        </enumeratedValue>
      </xsl:element>
    </xsl:copy>
  </xsl:template>

  <!-- Add the PASSWD enumeration to PMC[CKGR_MOR] register's KEY field -->
  <xsl:template match="/device/peripherals/peripheral[name='PMC']/registers/register[name='CKGR_MOR']/fields/field[name='KEY']">
    <!-- Copy the element -->
    <xsl:copy>
      <!-- And everything inside it -->
      <xsl:apply-templates select="@* | *"/> 
      <!-- Add new node for the enumeratedValues -->
      <xsl:element name="enumeratedValues">
        <enumeratedValue>
          <name>PASSWD</name>
          <description>Writing any other value in this field aborts the write operation.</description>
          <value>0x37</value>
        </enumeratedValue>
      </xsl:element>
    </xsl:copy>
  </xsl:template>
</xsl:stylesheet>
