import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static org.assertj.core.api.Assertions.*
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS

hitApi = WS.sendRequestAndVerify(findTestObject('cekNik', [('nik') : 'ID-C4702246']))

assertThat(hitApi.getResponseText()).contains('"tgl_kasus_positif": "2022-02-07"')
assertThat(hitApi.getResponseText()).contains('BLACK')
assertThat(hitApi.getResponseText()).doesNotContain('GREEN')
