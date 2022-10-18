class BaseTest(object):
    @classmethod
    def setup_class(cls):
        print("setup class")

    @classmethod
    def teardown_class(cls):
        print("teardown class")

    def setup_method(self, method: str):
        print("setup method: {}".format(method))

    def teardown_method(self, method: str):
        print("teardown method: {}".format(method))
