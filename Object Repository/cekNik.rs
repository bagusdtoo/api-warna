<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>cekNik</name>
   <tag></tag>
   <elementGuidId>0a84fa95-6128-49e7-8a15-f826d4cdbd04</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;nik&quot;,
      &quot;value&quot;: &quot;${nik}&quot;
    },
    {
      &quot;name&quot;: &quot;nama&quot;,
      &quot;value&quot;: &quot;Fulan&quot;
    },
    {
      &quot;name&quot;: &quot;telp&quot;,
      &quot;value&quot;: &quot;0&quot;
    },
    {
      &quot;name&quot;: &quot;latitude&quot;,
      &quot;value&quot;: &quot;0&quot;
    },
    {
      &quot;name&quot;: &quot;longitude&quot;,
      &quot;value&quot;: &quot;0&quot;
    },
    {
      &quot;name&quot;: &quot;placeId&quot;,
      &quot;value&quot;: &quot;000000000000000000000000&quot;
    }
  ]
}</httpBodyContent>
   <httpBodyType>x-www-form-urlencoded</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/x-www-form-urlencoded</value>
      <webElementGuid>1ae609ed-0be0-487e-892b-c89d00485467</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Api-Kemkes</name>
      <type>Main</type>
      <value>LWZlUEt9d0IrP0NQInknaDxIZk0mfmc1dE46WFYjdCU3XFNkKzJ0JQ==</value>
      <webElementGuid>418d6b3a-20e7-4a5d-b777-08da6b14f23e</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://api2-dev.dto.kemkes.go.id/api/check_nik2</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>dd38e414-6a79-4503-bd1a-8a0f618c8843</id>
      <masked>false</masked>
      <name>nik</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
