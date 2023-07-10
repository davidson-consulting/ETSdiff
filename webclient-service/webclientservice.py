import os
import time
import json
from selenium import webdriver
from selenium.webdriver.common.by import By
from selenium.webdriver.common.keys import Keys
from selenium.webdriver.support import expected_conditions
from selenium.webdriver.support.wait import WebDriverWait

class WebClientService():
    def __init__(self, scenario, file):
        self.before_test_actions = []
        self.run_test_actions = []
        self.after_test_actions = []
        self.load_scenario(scenario)

        self.file = file
        self.file_prepare = file + '.prepare'
        self.file_run = file + '.run'
        self.file_release = file + '.release'

        self.driver = webdriver.Firefox()

        self.prepare()
        self.run()
        self.release()

        self.driver.quit()

    def play_scenario(self, commands):
        for command in commands:
            #print(f'{command["command"]} for {command["target"]} -> {command["value"]}')
            if command['command'] == 'open':
                self.driver.get(self.scenario_url + command['target'])
            elif command['command'] == 'setWindowSize':
                window_size = command['target'].split('x')
                self.driver.set_window_size(window_size[0], window_size[1])
            elif command['command'] == 'click':
                target = command['target'].split('=')
                if target[0] == 'id':
                    WebDriverWait(self.driver, 30).until(expected_conditions.visibility_of_element_located((By.ID, target[1])))
                    self.driver.find_element(By.ID, target[1]).click()
                elif target[0] == 'css':
                    WebDriverWait(self.driver, 30).until(expected_conditions.visibility_of_element_located((By.CSS_SELECTOR, target[1])))
                    self.driver.find_element(By.CSS_SELECTOR, target[1]).click()
                elif target[0] == 'linkText':
                    WebDriverWait(self.driver, 30).until(expected_conditions.visibility_of_element_located((By.LINK_TEXT, target[1])))
                    self.driver.find_element(By.LINK_TEXT, target[1]).click()
                else:
                    print(f'unknow selector for {target[0]}')
                wait_for_ajax(self.driver)
            elif command['command'] == 'type':
                target = command['target'].split('=')
                if target[0] == 'id':
                    WebDriverWait(self.driver, 30).until(expected_conditions.visibility_of_element_located((By.ID, target[1])))
                    self.driver.find_element(By.ID, target[1]).send_keys(command['value'])
                else:
                    print(f'unknow selector for {target[0]}')
                wait_for_ajax(self.driver)
            elif command['command'] == 'sendKeys':
                target = command['target'].split('=')
                value = command['value'].replace('${KEY_', '').replace('}', '')
                if target[0] == 'id':
                    WebDriverWait(self.driver, 30).until(expected_conditions.visibility_of_element_located((By.ID, target[1])))
                    self.driver.find_element(By.ID, target[1]).send_keys(eval('Keys.' + value))
                else:
                    print(f'unknow selector for {target[0]}')
                wait_for_ajax(self.driver)
            else:
                print(f'unknow command {command["command"]}')

    def prepare(self):
        if len(self.before_test_actions) > 0:
            wait_for_file(self.file_prepare)
            print("prepare")
            self.play_scenario(self.before_test_actions)
            os.remove(self.file_prepare)

    def run(self):
        wait_for_file(self.file_run)
        print("run")
        self.play_scenario(self.run_test_actions)
        os.remove(self.file_run)

    def release(self):
        if len(self.after_test_actions) > 0:
            wait_for_file(self.file_release)
            print("release")
            self.play_scenario(self.after_test_actions)
            os.remove(self.file_release)

    def load_scenario(self, scenario):
        f = open(scenario)
        self.scenario = json.load(f)
        f.close

        if self.scenario['url']:
            self.scenario_url = self.scenario['url']
        else:
            sys.exit(f'ERROR: can\'t find scenario url')

        used_commands = {}
        actions = []
        for test in self.scenario['tests']:
            for command in test['commands']:
                if command['comment'] == 'BeforeTEST':
                    self.before_test_actions = actions
                    actions = []
                elif command['comment'] == 'AfterTEST':
                    self.run_test_actions = actions
                    actions = []
                else:
                    actions.append(command)

        if len(self.run_test_actions) > 0:
            self.after_test_actions = actions
        else:
            self.run_test_actions = actions

#        print(self.scenario_url)

#        print(self.before_test_actions)
#        print('--')
#        print(self.run_test_actions)
#        print('--')
#        print(self.after_test_actions)
#        sys.exit('stop')

def wait_for_file(file):
    while not os.path.exists(file):
        time.sleep(1)

def wait_for_ajax(driver):
    wait = WebDriverWait(driver, 15)
    try:
        wait.until(lambda driver: driver.execute_script('return jQuery.active') == 0)
        wait.until(lambda driver: driver.execute_script('return document.readyState') == 'complete')
    except Exception as e:
        pass

if __name__ == '__main__':
    import sys
    if len(sys.argv) < 3:
        sys.exit(f'ERROR: usage: {sys.argv[0]} <selenium.side> <waiting_file>')
    wcs = WebClientService(sys.argv[1], sys.argv[2])
