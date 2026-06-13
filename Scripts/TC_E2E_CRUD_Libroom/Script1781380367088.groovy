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

WebUI.openBrowser('')

WebUI.openBrowser('')

WebUI.navigateToUrl('http://localhost/pblperpustakaan/')

WebUI.click(findTestObject('Page_Rudy Ruang Study/body_Beranda'))

WebUI.click(findTestObject('Page_Rudy Ruang Study/a_Login'))

WebUI.setText(findTestObject('Page_Masuk - Rudy Ruang Study/input_Masukkan NIM_NIP'), 'adminrudy2')

WebUI.click(findTestObject('Page_Masuk - Rudy Ruang Study/input_Masukkan Password'))

WebUI.setEncryptedText(findTestObject('Page_Masuk - Rudy Ruang Study/input_Masukkan Password'), '8SQVv/p9jVSetdAvTyxx2A446CoqXiBG')

WebUI.sendKeys(findTestObject('Page_Masuk - Rudy Ruang Study/input_Masukkan Password'), 'Keys.chord(Keys.ENTER)')

WebUI.click(findTestObject('Page_Dashboard Admin - Rudy/a_Data Ruangan'))

WebUI.click(findTestObject('Page_Data Ruangan - Rudy/button_Tambah Ruangan'))

WebUI.setText(findTestObject('Page_Tambah Ruangan - Rudy/input_nama_ruangan'), 'Ruangan Ceria')

WebUI.setText(findTestObject('Page_Tambah Ruangan - Rudy/input_kapasitas_min'), '2')

WebUI.setText(findTestObject('Page_Tambah Ruangan - Rudy/input_kapasitas_max'), '4')

WebUI.setText(findTestObject('Page_Tambah Ruangan - Rudy/textarea_Tulis deskripsi ruangan'), 'Ruangan dengan Ac dan Smart TV')

WebUI.setText(findTestObject('Page_Tambah Ruangan - Rudy/input_gambar_ruangan'), 'C:\\fakepath\\e$kj#w.jpeg')

WebUI.click(findTestObject('Page_Tambah Ruangan - Rudy/button_Simpan'))

WebUI.click(findTestObject('Page_Data Ruangan - Rudy/a_Ubah'))

WebUI.setText(findTestObject('Page_Edit Ruangan - Rudy/input_kapasitas_max'), '8')

WebUI.setText(findTestObject('Page_Edit Ruangan - Rudy/textarea_Tulis deskripsi ruangan'), 'Sedang dalam perbaikan')

WebUI.selectOptionByValue(findTestObject('Page_Edit Ruangan - Rudy/select_status'), 'Tidak Tersedia', false)

WebUI.click(findTestObject('Page_Edit Ruangan - Rudy/button_Simpan Perubahan'))

WebUI.click(findTestObject('Page_Data Ruangan - Rudy/button_Hapus'))

WebUI.click(findTestObject('Page_Data Ruangan - Rudy/button_Ya, hapus'))

