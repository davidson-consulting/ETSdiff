# Generated by Selenium IDE
import os
import time
from selenium import webdriver
from selenium.webdriver.common.by import By
from selenium.webdriver.common.action_chains import ActionChains
from selenium.webdriver.support import expected_conditions
from selenium.webdriver.support.wait import WebDriverWait
from selenium.webdriver.common.keys import Keys
from selenium.webdriver.common.desired_capabilities import DesiredCapabilities

class Test():
  def __init__(self, file):
    self.file = file
    self.file_prepare = file + '.prepare'
    self.file_run = file + '.run'
    self.file_release = file + '.release'

    self.driver = webdriver.Firefox()
    self.driver.set_window_size(1280, 1024)
    self.vars = {}

  def prepare(self):
    self.driver.get("http://localhost/login")
    self.driver.find_element(By.ID, "username").click()
    self.driver.find_element(By.ID, "username").send_keys("admin@davidson.fr")
    self.driver.find_element(By.ID, "password").send_keys("*****")
    self.driver.find_element(By.ID, "_submit").click()
    WebDriverWait(self.driver, 30).until(expected_conditions.visibility_of_element_located((By.CSS_SELECTOR, ".btn-dropdown-add")))
    os.remove(self.file_prepare)

  def release(self):
    self.driver.find_element(By.CSS_SELECTOR, ".dropdown-toggle:nth-child(2)").click()
    self.driver.find_element(By.LINK_TEXT, "Supprimer").click()
    element = self.driver.find_element(By.CSS_SELECTOR, ".btn:nth-child(3)")
    actions = ActionChains(self.driver)
    actions.move_to_element(element).perform()
    element = self.driver.find_element(By.CSS_SELECTOR, "body")
    actions = ActionChains(self.driver)
    #actions.move_to_element(element, 0, 0).perform() shama MDF
    actions.move_to_element(element).perform()
    self.driver.find_element(By.LINK_TEXT, "Admin Admin").click()
    self.driver.find_element(By.LINK_TEXT, "Déconnexion").click()

    WebDriverWait(self.driver, 30).until(expected_conditions.visibility_of_element_located((By.ID, "username")))
    #self.driver.close()

    #self.driver.quit()
    os.remove(self.file_release)

  def run(self):
    self.driver.find_element(By.CSS_SELECTOR, ".btn-dropdown-add").click()
    self.driver.find_element(By.CSS_SELECTOR, "li:nth-child(4) span").click()
    self.driver.find_element(By.ID, "subcontractor_company_registration_companyName").click()
    self.driver.find_element(By.ID, "subcontractor_company_registration_companyName").send_keys("ETSDiff-TEST")
    self.driver.find_element(By.ID, "subcontractor_company_registration_siren").send_keys("123454321")
    self.driver.find_element(By.ID, "subcontractor_company_registration_siren_search_button").click()
    WebDriverWait(self.driver, 30).until(expected_conditions.visibility_of_element_located((By.ID, "subcontractor_company_registration_siret")))
    #time.sleep(2) # shama MDF
    self.driver.find_element(By.ID, "subcontractor_company_registration_companyName").click()
    self.driver.find_element(By.ID, "subcontractor_company_registration_companyName").send_keys("ETSDiff-TEST")
    self.driver.find_element(By.ID, "subcontractor_company_registration_siren").send_keys("123454321")
    self.driver.find_element(By.ID, "subcontractor_company_registration_siret").send_keys("12321")
    self.driver.find_element(By.ID, "subcontractor_company_registration_apeCode").send_keys("NA")
    self.driver.find_element(By.ID, "subcontractor_company_registration_address").send_keys("1 rue test")
    self.driver.find_element(By.ID, "subcontractor_company_registration_postalCode").send_keys("1111")
    self.driver.find_element(By.ID, "subcontractor_company_registration_city").send_keys("Test")
    self.driver.find_element(By.ID, "subcontractor_company_registration_country").send_keys("France")
    self.driver.find_element(By.ID, "subcontractor_company_registration_accountingContactFirstName").send_keys("ETS")
    self.driver.find_element(By.ID, "subcontractor_company_registration_accountingContactLastName").send_keys("Test")
    self.driver.find_element(By.ID, "subcontractor_company_registration_accountingContactEmail").send_keys("ets@test.fr")
    self.driver.find_element(By.CSS_SELECTOR, "fieldset:nth-child(3) > .row-fluid:nth-child(3)").click()
    self.driver.find_element(By.CSS_SELECTOR, ".form-actions > .btn").click()
    WebDriverWait(self.driver, 30).until(expected_conditions.visibility_of_element_located((By.CSS_SELECTOR, ".dropdown-toggle:nth-child(2)")))

    os.remove(self.file_run)

def wait_for_file(file):
    while not os.path.exists(file):
        time.sleep(1)

if __name__ == '__main__':
    import sys
    if len(sys.argv) < 2:
        sys.exit("ERROR: missing filename argument")
    test = Test(sys.argv[1])
    while True:
        wait_for_file(test.file_prepare)
        test.prepare()
        wait_for_file(test.file_run)
        test.run()
        wait_for_file(test.file_release)
        test.release()

#class TestCompagnysubcontratorcreateanddelete():
#  def setup_method(self, method):
#    self.driver = webdriver.Firefox()
#    self.vars = {}
#  
#  def teardown_method(self, method):
#    self.driver.quit()
#  
#  def test_compagnysubcontratorcreateanddelete(self):
#    self.driver.get("http://localhost/login")
#    self.driver.set_window_size(1114, 772)
#    self.driver.find_element(By.ID, "username").click()
#    self.driver.find_element(By.ID, "username").send_keys("admin@davidson.fr")
#    self.driver.find_element(By.ID, "password").send_keys("admin")
#    self.driver.find_element(By.ID, "_submit").click()
#
#    self.driver.find_element(By.CSS_SELECTOR, ".btn-dropdown-add").click()
#    self.driver.find_element(By.CSS_SELECTOR, "li:nth-child(4) span").click()
#    self.driver.find_element(By.ID, "subcontractor_company_registration_companyName").click()
#    self.driver.find_element(By.ID, "subcontractor_company_registration_companyName").send_keys("ETSdiff")
#    self.driver.find_element(By.ID, "subcontractor_company_registration_siren").send_keys("111")
#    self.driver.find_element(By.ID, "subcontractor_company_registration_siren_search_button").click()
#    time.sleep(2) # shama MDF
#    self.driver.find_element(By.ID, "subcontractor_company_registration_companyName").click()
#    self.driver.find_element(By.ID, "subcontractor_company_registration_companyName").send_keys("ETSdiff")
#    self.driver.find_element(By.ID, "subcontractor_company_registration_siren").send_keys("111")
#    self.driver.find_element(By.ID, "subcontractor_company_registration_siret").send_keys("1")
#    self.driver.find_element(By.ID, "subcontractor_company_registration_apeCode").send_keys("1")
#    self.driver.find_element(By.ID, "subcontractor_company_registration_address").send_keys("1 rue test")
#    self.driver.find_element(By.ID, "subcontractor_company_registration_postalCode").send_keys("1111")
#    self.driver.find_element(By.ID, "subcontractor_company_registration_city").send_keys("Test")
#    self.driver.find_element(By.ID, "subcontractor_company_registration_country").send_keys("France")
#    self.driver.find_element(By.ID, "subcontractor_company_registration_accountingContactFirstName").send_keys("test")
#    self.driver.find_element(By.ID, "subcontractor_company_registration_accountingContactLastName").send_keys("ets")
#    self.driver.find_element(By.ID, "subcontractor_company_registration_accountingContactEmail").send_keys("test@ets.fr")
#    self.driver.find_element(By.CSS_SELECTOR, "fieldset:nth-child(3) > .row-fluid:nth-child(3)").click()
#    self.driver.find_element(By.CSS_SELECTOR, ".form-actions > .btn").click()
#
#    self.driver.find_element(By.CSS_SELECTOR, ".dropdown-toggle:nth-child(2)").click()
#    self.driver.find_element(By.LINK_TEXT, "Supprimer").click()
#    element = self.driver.find_element(By.CSS_SELECTOR, ".btn:nth-child(3)")
#    actions = ActionChains(self.driver)
#    actions.move_to_element(element).perform()
#    element = self.driver.find_element(By.CSS_SELECTOR, "body")
#    actions = ActionChains(self.driver)
#    #actions.move_to_element(element, 0, 0).perform() shama MDF
#    actions.move_to_element(element).perform()
#    self.driver.find_element(By.LINK_TEXT, "Admin Admin").click()
#    self.driver.find_element(By.LINK_TEXT, "Déconnexion").click()
#  