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

response = WS.sendRequest(findTestObject('Booking/Create Booking'))

WS.verifyElementPropertyValue(response, 'booking.firstname', "Rian")
WS.verifyElementPropertyValue(response, 'booking.lastname', "Gunawan")
WS.verifyElementPropertyValue(response, 'booking.totalprice', 320)
WS.verifyElementPropertyValue(response, 'booking.depositpaid', false)
WS.verifyElementPropertyValue(response, 'booking.bookingdates.checkin', "2024-02-19")
WS.verifyElementPropertyValue(response, 'booking.bookingdates.checkout', "2024-02-28")
WS.verifyElementPropertyValue(response, 'booking.additionalneeds', "Dinner")

GlobalVariable.bookingId = WS.getElementText(response, 'bookingid')
println GlobalVariable.bookingId