import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

createAuth = WS.sendRequest(findTestObject('Auth/Create Authentication'))

GlobalVariable.token = WS.getElementPropertyValue(createAuth, 'token')

booking = WS.sendRequest(findTestObject('Booking/Create Booking'))

GlobalVariable.bookingId = WS.getElementPropertyValue(booking, 'bookingid')

response = WS.sendRequestAndVerify(findTestObject('Booking/Update Booking'))

WS.verifyElementPropertyValue(response, 'firstname', "James")
WS.verifyElementPropertyValue(response, 'lastname', "Brown")
WS.verifyElementPropertyValue(response, 'totalprice', 111)
WS.verifyElementPropertyValue(response, 'depositpaid', true)
WS.verifyElementPropertyValue(response, 'bookingdates.checkin', "2023-04-20")
WS.verifyElementPropertyValue(response, 'bookingdates.checkout', "2023-06-09")
WS.verifyElementPropertyValue(response, 'additionalneeds', "Breakfast")
