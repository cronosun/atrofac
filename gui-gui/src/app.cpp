#include "app.hpp"

fc::CurveEditor::CurveEditor(std::string name)
    : Fl_Flow(0, 0, 1, 1)
{
    box(fl_ext_box(BTN_UP_BOX));
    applied = std::make_unique<Fl_Hover_Ext<Fl_Toggle_Button>>(0, 0, 1, 25);
    applied->normal_box(fl_ext_box(BTN_UP_BOX));
    applied->down_box(fl_ext_box(BTN_DOWN_BOX));
    applied->hover_box(fl_ext_box(BTN_HOVER_BOX));
    applied->labelsize(sdpi(applied->labelsize()));
    applied->copy_label(name.c_str());
    add(applied.get());
    rule(applied.get(), "=<^");

    for (int i = 0; i < 8; i++)
    {
        sliders[i] = std::make_unique<Fl_Ext<Fl_Value_Slider>>(0, 0, 1, 25);
        sliders[i]->type(FL_HOR_FILL_SLIDER);
        sliders[i]->slider(FL_FLAT_BOX);
        sliders[i]->box(FL_FLAT_BOX);
        sliders[i]->color(Fl_Ext_Color(0xF1F1F1));
        sliders[i]->color2(Fl_Ext_Color(0xC1C1C1));
        sliders[i]->textsize(sdpi(sliders[i]->textsize()));
        add(sliders[i].get());
        rule(sliders[i].get(), "=<^");
    }
}

fc::App::App(int W, int H, const char* title) //
    : Fl_Win_Ext<Fl_Double_Window>(100, 100, W, H, title)
{
    Fl::visible_focus(0);
    init();
    Style_Initializer();
}

fc::App::~App()
{
    exit();
}

void fc::App::init()
{
    profiles[0] = std::make_unique<Fl_Hover_Ext<Fl_Radio_Button>>(0, 0, 1, 50, "silent");
    profiles[1] = std::make_unique<Fl_Hover_Ext<Fl_Radio_Button>>(0, 0, 1, 50, "windows");
    profiles[2] = std::make_unique<Fl_Hover_Ext<Fl_Radio_Button>>(0, 0, 1, 50, "performance");
    profiles[3] = std::make_unique<Fl_Hover_Ext<Fl_Radio_Button>>(0, 0, 1, 50, "turbo");

    static Flcb profile_func = [&](Fl_Widget* w) { curr_profile = w->label(); };
    for (int i = 0; i < 4; i++)
    {
        profiles[i]->labelsize(sdpi(profiles[i]->labelsize()));
        profiles[i]->normal_box(fl_ext_box(BTN_UP_BOX));
        profiles[i]->down_box(fl_ext_box(BTN_DOWN_BOX));
        profiles[i]->hover_box(fl_ext_box(BTN_HOVER_BOX));
        profiles[i]->attrib.callback(profile_func);
        add(profiles[i].get());
    }
    profiles[1]->value(1);

    rule(profiles[0].get(), "/<^=<");
    rule(profiles[1].get(), "^>=<");
    rule(profiles[2].get(), "/<^=<");
    rule(profiles[3].get(), "^>=<");

    ce = std::make_unique<CurveEditor>("CPU");
    ge = std::make_unique<CurveEditor>("GPU");
    add(ce.get());
    add(ge.get());

    temps[0] = std::make_unique<Fl_Ext<Fl_Box>>(0, 0, 50, 25);
    temps[0]->labelsize(sdpi(temps[0]->labelsize()));
    temps[0]->box(FL_FLAT_BOX);
    add(temps[0].get());
    rule(temps[0].get(), "/<^");

    for (int i = 1; i < 9; i++)
    {
        temps[i] = std::make_unique<Fl_Ext<Fl_Box>>(0, 0, 50, 25);
        temps[i]->copy_label((std::to_string(30 + (i - 1) * 10) + "c").c_str());
        temps[i]->labelsize(sdpi(temps[i]->labelsize()));
        temps[i]->box(fl_ext_box(BTN_UP_BOX));
        add(temps[i].get());
        rule(temps[i].get(), "/<^");
    }

    rule(ce.get(), "<^=>");
    rule(ge.get(), "^<=>");

    static Flcb applied_func = [&](Fl_Widget* w)
    {
        std::string cmd = "atrofac-cli fan";
        cmd += std::format(" --plan {}", curr_profile);

        if (ce->applied->value() == true)
        {
            std::string cpu_cmd = std::format(" --cpu 30c:{}%", int(ce->sliders[0]->value() * 100));
            for (int i = 1; i < 8; i++)
            {
                cpu_cmd += std::format(",{}c:{}%", 30 + i * 10, int(ce->sliders[i]->value() * 100));
            }
            cmd += cpu_cmd;
        }

        if (ge->applied->value() == true)
        {
            std::string gpu_cmd = std::format(" --gpu 30c:{}%", int(ge->sliders[0]->value() * 100));
            for (int i = 1; i < 8; i++)
            {
                gpu_cmd += std::format(",{}c:{}%", 30 + i * 10, int(ge->sliders[i]->value() * 100));
            }
            cmd += gpu_cmd;
        }

        std::system(cmd.c_str());
    };

    applied = std::make_unique<Fl_Hover_Ext<Fl_Button>>(0, 0, 1, 50, "Applied");
    applied->labelsize(sdpi(applied->labelsize()));
    applied->normal_box(fl_ext_box(BTN_UP_BOX));
    applied->down_box(fl_ext_box(BTN_DOWN_BOX));
    applied->hover_box(fl_ext_box(BTN_HOVER_BOX));
    applied->attrib.callback(applied_func);
    add(applied.get());
    rule(applied.get(), "=<");

    rule(ce.get(), "=v");
    rule(ge.get(), "=v");
}

void fc::App::exit() {}

int fc::App::run(int argc, char** argv)
{
    show();
    return Fl::run();
}
