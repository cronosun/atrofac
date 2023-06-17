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

    curve = std::make_unique<PlotGraph>(70, 0, 0, 1, 1);
    curve->labelsize(sdpi(curve->labelsize()));
    curve->thickness_ = sdpi(2);
    curve->box(fl_ext_box(BTN_UP_BOX));
    curve->color2(Fl_Ext_Color(0x005499));
    add(curve.get());
    rule(curve.get(), "=<=^");
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

    rule(ce.get(), "<^/>=<");
    rule(ge.get(), "^=<");

    static Flcb applied_func = [&](Fl_Widget* w)
    {
        std::string cmd = "atrofac-cli.exe fan";
        cmd += std::format(" --plan {}", curr_profile);

        if (ce->applied->value() == true)
        {
            std::string cpu_cmd = std::format(" --cpu ");
            for (int i = 0; i < 7; i++)
            {
                int sum = 0;
                for (int k = 0; k < 10; k++)
                {
                    sum += ce->curve->table_[i * 10 + k];
                }
                sum /= 10;
                cpu_cmd += std::format("{}c:{}%,", 30 + i * 10, sum);
            }
            cpu_cmd += std::format("{}c:{}%", 100, ce->curve->table_[ce->curve->slices_]);
            cmd += cpu_cmd;
        }

        if (ge->applied->value() == true)
        {
            std::string gpu_cmd = std::format(" --gpu ");
            for (int i = 0; i < 7; i++)
            {
                int sum = 0;
                for (int k = 0; k < 10; k++)
                {
                    sum += ge->curve->table_[i * 10 + k];
                }
                sum /= 10;
                gpu_cmd += std::format("{}c:{}%,", 30 + i * 10, sum);
            }
            gpu_cmd += std::format("{}c:{}%", 100, ge->curve->table_[ge->curve->slices_]);
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

fc::PlotGraph::PlotGraph(int slices, int x, int y, int w, int h)
    : Fl_Widget(x, y, w, h),
      slices_(slices)
{
    table_ = new int[slices + 1]{};
}

fc::PlotGraph::~PlotGraph()
{
    delete[] table_;
}

void fc::PlotGraph::update_data()
{
    int x_entry = (slices_ + 1) * (Fl::event_x() - x()) / float(w());
    int y_entry = 100 - (100 + 1) * (Fl::event_y() - y()) / float(h());
    x_entry = std::clamp(x_entry, 0, slices_);
    y_entry = std::clamp(y_entry, 0, 100);
    table_[x_entry] = y_entry;

    copy_label(std::format("{}c, {}%", 30 + x_entry, y_entry).c_str());

    redraw();
}

int fc::PlotGraph::handle(int event)
{
    switch (event)
    {
        case FL_DRAG:
        {
            update_data();
            break;
        }
        case FL_PUSH:
        {
            update_data();
            break;
        }
    }

    return 1;
}

void fc::PlotGraph::draw()
{
    draw_box();

    fl_color(color2());
    fl_line_style(line_style_, thickness_);
    fl_begin_line();
    for (int i = 1; i < slices_ + 2; i++)
    {
        double vert_x = x() + (i * double(w()) / (slices_ + 2));
        double vert_y = y() + h() * (1 - double(table_[i - 1]) / (100 + 2) - 0.01);
        fl_vertex(vert_x, vert_y);
    }
    fl_end_line();

    draw_label();
}
