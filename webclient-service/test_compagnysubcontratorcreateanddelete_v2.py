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
    self.driver.get("http://localhost:3000/login")
    self.driver.find_element(By.CSS_SELECTOR, "#login_normal > span").click()
    WebDriverWait(self.driver, 30).until(expected_conditions.visibility_of_element_located((By.ID, "input-username")))

    self.driver.find_element(By.ID, "input-username").send_keys("vincent.cagnard")
    self.driver.find_element(By.ID, "input-password").send_keys("*****")
    self.driver.find_element(By.CSS_SELECTOR, "span").click()
    WebDriverWait(self.driver, 30).until(expected_conditions.visibility_of_element_located((By.CSS_SELECTOR, ".btn-nav-outline:nth-child(2) > span")))
    os.remove(self.file_prepare)
  
  def release(self):
    self.driver.find_element(By.CSS_SELECTOR, "#mobile-btn #icon").click()
    self.driver.find_element(By.CSS_SELECTOR, "#mobile-btn > #entity-dropdown #dropdown_item_delete > div").click()
    WebDriverWait(self.driver, 30).until(expected_conditions.visibility_of_element_located((By.CSS_SELECTOR, "#button-form-validation > span")))

    self.driver.find_element(By.CSS_SELECTOR, "#button-form-validation > span").click()
    self.driver.find_element(By.ID, "user-account-btn").click()
    self.driver.find_element(By.CSS_SELECTOR, "#dropdown_item_header_user_account_menu_logout > div").click()
    #self.driver.close()

    #self.driver.quit()
    os.remove(self.file_release)
  
  def run(self):
    self.driver.find_element(By.CSS_SELECTOR, ".btn-nav-outline:nth-child(2) > span").click()
    self.driver.find_element(By.CSS_SELECTOR, "#dropdown_item_add_str_company > div").click()
    self.driver.find_element(By.ID, "input-text-company_name").click()
    self.driver.find_element(By.ID, "input-text-company_name").send_keys("ETSDiff-TEST")
    self.driver.find_element(By.ID, "input-text-search-siren").send_keys("123454321")
    self.driver.find_element(By.ID, "search-icon").click()
    WebDriverWait(self.driver, 30).until(expected_conditions.visibility_of_element_located((By.ID, "input-text-siret")))

    self.driver.find_element(By.ID, "input-text-siret").click()
    self.driver.find_element(By.ID, "input-text-siret").send_keys("12321")
    self.driver.find_element(By.ID, "input-text-auxiliary_code").send_keys("NA")
    self.driver.find_element(By.ID, "input-text-country").send_keys("France")
    self.driver.find_element(By.ID, "input-text-city").send_keys("Test")
    self.driver.find_element(By.ID, "input-text-postal_code").send_keys("1111")
    self.driver.find_element(By.ID, "input-area-input_address_label").send_keys("1 rue test")
    self.driver.find_element(By.ID, "input-text-first_name").send_keys("ETS")
    self.driver.find_element(By.ID, "input-text-last_name").send_keys("Test")
    self.driver.find_element(By.ID, "input-text-email").send_keys("ets@test.fr")
    self.driver.find_element(By.ID, "button-form-validation").click()
    WebDriverWait(self.driver, 30).until(expected_conditions.visibility_of_element_located((By.CSS_SELECTOR, "#button-form-close > span")))

    self.driver.find_element(By.CSS_SELECTOR, "#button-form-close > span").click()
    WebDriverWait(self.driver, 30).until(expected_conditions.visibility_of_element_located((By.CSS_SELECTOR, "#mobile-btn #icon")))
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