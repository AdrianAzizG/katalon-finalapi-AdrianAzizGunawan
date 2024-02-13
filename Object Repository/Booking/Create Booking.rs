<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Create Booking</name>
   <tag></tag>
   <elementGuidId>8085b5b3-479e-47c4-8ccb-79d863ecf3f4</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;firstname\&quot; : \&quot;${firstName}\&quot;,\n    \&quot;lastname\&quot; : \&quot;${lastName}\&quot;,\n    \&quot;totalprice\&quot; : ${price},\n    \&quot;depositpaid\&quot; : ${depositPaid},\n    \&quot;bookingdates\&quot; : {\n        \&quot;checkin\&quot; : \&quot;${checkIn}\&quot;,\n        \&quot;checkout\&quot; : \&quot;${checkOut}\&quot;\n    },\n    \&quot;additionalneeds\&quot; : \&quot;${additionalNeeds}\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>8211b738-b0cd-4b40-ad62-9f6e0e52e7b2</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>e1c9bc81-bd28-4d95-a2c9-674fcd6cb240</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.3.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${GlobalVariable.url}/booking</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'Rian'</defaultValue>
      <description></description>
      <id>d49e9e9e-0e6e-433f-bf0a-fce3317ca8a1</id>
      <masked>false</masked>
      <name>firstName</name>
   </variables>
   <variables>
      <defaultValue>'Gunawan'</defaultValue>
      <description></description>
      <id>90c6941d-69ad-412c-834d-bdbae5d5be55</id>
      <masked>false</masked>
      <name>lastName</name>
   </variables>
   <variables>
      <defaultValue>320</defaultValue>
      <description></description>
      <id>8d88eef7-1634-43b9-92f4-7154f0ba6f52</id>
      <masked>false</masked>
      <name>price</name>
   </variables>
   <variables>
      <defaultValue>false</defaultValue>
      <description></description>
      <id>36c6a013-ed2c-4f74-8ea0-46a869022969</id>
      <masked>false</masked>
      <name>depositPaid</name>
   </variables>
   <variables>
      <defaultValue>'2024-02-19'</defaultValue>
      <description></description>
      <id>6527e1d0-7ef4-49b6-b0bd-df11492aabfb</id>
      <masked>false</masked>
      <name>checkIn</name>
   </variables>
   <variables>
      <defaultValue>'2024-02-28'</defaultValue>
      <description></description>
      <id>67dd67c0-451e-4920-b285-b090d4d4e956</id>
      <masked>false</masked>
      <name>checkOut</name>
   </variables>
   <variables>
      <defaultValue>'Dinner'</defaultValue>
      <description></description>
      <id>1aae37c9-068a-4acb-9e6b-37e08ca16882</id>
      <masked>false</masked>
      <name>additionalNeeds</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

WS.verifyElementPropertyValue(response, 'booking.firstname', &quot;Rian&quot;)
WS.verifyElementPropertyValue(response, 'booking.lastname', &quot;Gunawan&quot;)
WS.verifyElementPropertyValue(response, 'booking.totalprice', 320)
WS.verifyElementPropertyValue(response, 'booking.depositpaid', false)
WS.verifyElementPropertyValue(response, 'booking.bookingdates.checkin', &quot;2024-02-19&quot;)
WS.verifyElementPropertyValue(response, 'booking.bookingdates.checkout', &quot;2024-02-28&quot;)
WS.verifyElementPropertyValue(response, 'booking.additionalneeds', &quot;Dinner&quot;)

GlobalVariable.bookingId = WS.getElementText(response, 'bookingid')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
